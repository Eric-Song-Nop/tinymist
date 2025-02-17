#![allow(missing_docs)]

use std::sync::Arc;

use reflexo_typst::Bytes;
use tinymist_project::{EntryReader, LspCompilerFeat, PdfExport, PngExport, SvgExport, TaskWhen};
use tinymist_std::error::prelude::*;
use tinymist_std::typst::{TypstDocument, TypstPagedDocument};
use tinymist_task::ExportTimings;
use typlite::Typlite;
use typst::diag::SourceResult;

use crate::project::{ExportMarkdownTask, ExportTextTask, ProjectTask};
use crate::tool::text::FullTextDigest;
use crate::world::base::{
    CompilerFeat, ConfigTask, DiagnosticsTask, ExportComputation, FlagTask, OptionDocumentTask,
    PagedCompilationTask, WorldComputable, WorldComputeGraph,
};

#[derive(Clone, Copy, Default)]
pub struct ProjectCompilation;

impl ProjectCompilation {
    pub fn preconfig_timings<F: CompilerFeat>(graph: &Arc<WorldComputeGraph<F>>) -> Result<bool> {
        // todo: configure run_diagnostics!
        let paged_diag = Some(TaskWhen::OnType);

        let pdf: Option<TaskWhen> = graph
            .get::<ConfigTask<<PdfExport as ExportComputation<LspCompilerFeat, _>>::Config>>()
            .transpose()?
            .map(|config| config.export.when);
        let svg: Option<TaskWhen> = graph
            .get::<ConfigTask<<SvgExport as ExportComputation<LspCompilerFeat, _>>::Config>>()
            .transpose()?
            .map(|config| config.export.when);
        let png: Option<TaskWhen> = graph
            .get::<ConfigTask<<PngExport as ExportComputation<LspCompilerFeat, _>>::Config>>()
            .transpose()?
            .map(|config| config.export.when);
        let md: Option<TaskWhen> = graph
            .get::<ConfigTask<ExportMarkdownTask>>()
            .transpose()?
            .map(|config| config.export.when);
        let text: Option<TaskWhen> = graph
            .get::<ConfigTask<<TextExport as ExportComputation<LspCompilerFeat, _>>::Config>>()
            .transpose()?
            .map(|config| config.export.when);

        let doc = None::<TypstPagedDocument>.as_ref();
        let check = |timing| ExportTimings::needs_run(&graph.snap, timing, doc).unwrap_or(true);

        let compile_paged = [paged_diag, pdf, svg, png, text, md].into_iter().any(check);

        let _ = graph.provide::<FlagTask<PagedCompilationTask>>(Ok(FlagTask::flag(compile_paged)));

        Ok(compile_paged)
    }
}

impl<F: CompilerFeat> WorldComputable<F> for ProjectCompilation {
    type Output = Self;

    fn compute(graph: &Arc<WorldComputeGraph<F>>) -> Result<Self> {
        Self::preconfig_timings(graph)?;
        DiagnosticsTask::compute(graph)?;
        Ok(Self)
    }
}

pub struct ProjectExport;

impl ProjectExport {
    fn export_bytes<T: ExportComputation<LspCompilerFeat, TypstPagedDocument, Output = Bytes>>(
        graph: &Arc<WorldComputeGraph<LspCompilerFeat>>,
        when: Option<TaskWhen>,
        config: &T::Config,
    ) -> Result<Option<Bytes>> {
        let doc = graph.compute::<OptionDocumentTask<TypstPagedDocument>>()?;
        let doc = doc.as_ref();
        let n = ExportTimings::needs_run(&graph.snap, when, doc.as_deref()).unwrap_or(true);
        if !n {
            return Ok(None);
        }

        let res = doc.as_ref().map(|doc| T::run(graph, doc, config));
        res.transpose()
    }

    fn export_string<T: ExportComputation<LspCompilerFeat, TypstPagedDocument, Output = String>>(
        graph: &Arc<WorldComputeGraph<LspCompilerFeat>>,
        when: Option<TaskWhen>,
        config: &T::Config,
    ) -> Result<Option<Bytes>> {
        let doc = graph.compute::<OptionDocumentTask<TypstPagedDocument>>()?;
        let doc = doc.as_ref();
        let n = ExportTimings::needs_run(&graph.snap, when, doc.as_deref()).unwrap_or(true);
        if !n {
            return Ok(None);
        }

        let doc = doc.as_ref();
        let res = doc.map(|doc| {
            T::run(graph, doc, config)
                .map(String::into_bytes)
                .map(Bytes::from)
        });
        res.transpose()
    }
}

impl WorldComputable<LspCompilerFeat> for ProjectExport {
    type Output = Self;

    fn compute(graph: &Arc<WorldComputeGraph<LspCompilerFeat>>) -> Result<Self> {
        let config = graph.must_get::<ConfigTask<ProjectTask>>()?;
        let output_path = config.as_export().and_then(|e| {
            e.output
                .as_ref()
                .and_then(|o| o.substitute(&graph.snap.world.entry_state()))
        });
        let when = config.when();

        let output = || -> Result<Option<Bytes>> {
            use ProjectTask::*;
            match config.as_ref() {
                Preview(..) => todo!(),
                ExportPdf(config) => Self::export_bytes::<PdfExport>(graph, when, config),
                ExportPng(config) => Self::export_bytes::<PngExport>(graph, when, config),
                ExportSvg(config) => Self::export_string::<SvgExport>(graph, when, config),
                ExportHtml(_config) => todo!(),
                ExportMd(_config) => {
                    let doc = graph.compute::<OptionDocumentTask<TypstPagedDocument>>()?;
                    let doc = doc.as_ref();
                    let n =
                        ExportTimings::needs_run(&graph.snap, when, doc.as_deref()).unwrap_or(true);
                    if !n {
                        return Ok(None);
                    }

                    Ok(TypliteMdExport::run(graph)?
                        .map(String::into_bytes)
                        .map(Bytes::from))
                }
                ExportText(config) => Self::export_string::<TextExport>(graph, when, config),
                Query(..) => todo!(),
            }
        };

        if let Some(path) = output_path {
            let output = output()?;
            // todo: don't ignore export source diagnostics
            if let Some(output) = output {
                std::fs::write(path, output).context("failed to write output")?;
            }
        }

        Ok(Self {})
    }
}

pub struct TypliteMdExport(pub Option<SourceResult<String>>);

impl TypliteMdExport {
    fn run(graph: &Arc<WorldComputeGraph<LspCompilerFeat>>) -> Result<Option<String>> {
        let conv = Typlite::new(Arc::new(graph.snap.world.clone()))
            .convert()
            .map_err(|e| anyhow::anyhow!("failed to convert to markdown: {e}"))?;

        Ok(Some(conv.to_string()))
    }
}

impl WorldComputable<LspCompilerFeat> for TypliteMdExport {
    type Output = Option<String>;

    fn compute(graph: &Arc<WorldComputeGraph<LspCompilerFeat>>) -> Result<Self::Output> {
        Self::run(graph)
    }
}

pub struct TextExport;

impl<F: CompilerFeat> ExportComputation<F, TypstPagedDocument> for TextExport {
    type Output = String;
    type Config = ExportTextTask;

    fn run(
        _g: &Arc<WorldComputeGraph<F>>,
        doc: &Arc<TypstPagedDocument>,
        _config: &ExportTextTask,
    ) -> Result<String> {
        Ok(format!(
            "{}",
            FullTextDigest(TypstDocument::Paged(doc.clone()))
        ))
    }
}
