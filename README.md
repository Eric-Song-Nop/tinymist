<!-- This file is generated by scripts/link-docs.mjs from docs/tinymist/introduction.typ. Do not edit manually. -->
# Tinymist

Tinymist [ˈtaɪni mɪst] is an integrated language service for [Typst](https://typst.app/) [taɪpst]. You can also call it "微霭" [wēi ǎi] in Chinese.

It contains:
- an analyzing library for Typst, see [tinymist-query](https://github.com/Myriad-Dreamin/tinymist/tree/main/crates/tinymist-query).
- a CLI for Typst, see [tinymist](https://github.com/Myriad-Dreamin/tinymist/tree/main/crates/tinymist/).
  - which provides a language server for Typst, see [Language Features](https://myriad-dreamin.github.io/tinymist//feature/language.html).
  - which provides a preview server for Typst, see [Preview Feature](https://myriad-dreamin.github.io/tinymist//feature/preview.html).
- a VSCode extension for Typst, see [Tinymist VSCode Extension](https://github.com/Myriad-Dreamin/tinymist/tree/main/editors/vscode/).

## Features




Language service (LSP) features:

- [Semantic highlighting](https://code.visualstudio.com/api/language-extensions/semantic-highlight-guide)
  - The "semantic highlighting" is supplementary to ["syntax highlighting"](https://code.visualstudio.com/api/language-extensions/syntax-highlight-guide).

- [Code actions](https://code.visualstudio.com/api/language-extensions/programmatic-language-features#provide-code-actions)
  - Also known as "quick fixes" or "refactorings".
- [Formatting (Reformatting)](https://code.visualstudio.com/api/language-extensions/programmatic-language-features#format-source-code-in-an-editor)
  - Provide the user with support for formatting whole documents, using [typstfmt](https://github.com/astrale-sharp/typstfmt) or [typstyle](https://github.com/Enter-tainer/typstyle).
- [Document highlight](https://code.visualstudio.com/api/language-extensions/programmatic-language-features#highlight-all-occurrences-of-a-symbol-in-a-document)
  - Highlight all break points in a loop context.
  - (Todo) Highlight all exit points in a function context.
  - (Todo) Highlight all captures in a closure context.
  - (Todo) Highlight all occurrences of a symbol in a document.
- [Document links](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_documentLink)
  - Renders path or link references in the document, such as `image("path.png")` or `bibliography(style: "path.csl")`.
- [Document symbols](https://code.visualstudio.com/docs/getstarted/userinterface#_outline-view)
  - Also known as "document outline" or "table of contents" _in Typst_.
- [Folding ranges](https://burkeholland.gitbook.io/vs-code-can-do-that/exercise-3-navigation-and-refactoring/folding-sections)
  - You can collapse code/content blocks and headings.
- [Goto definitions](https://code.visualstudio.com/api/language-extensions/programmatic-language-features#show-definitions-of-a-symbol)
  - Right-click on a symbol and select "Go to Definition".
  - Or ctrl+click on a symbol.
- [References](https://code.visualstudio.com/api/language-extensions/programmatic-language-features#find-all-references-to-a-symbol)
  - Right-click on a symbol and select "Go to References" or "Find References".
  - Or ctrl+click on a symbol.
- [Hover tips](https://code.visualstudio.com/api/language-extensions/programmatic-language-features#show-hovers)
  - Also known as "hovering tooltip".
  - Render docs according to [tidy](https://github.com/Mc-Zen/tidy) style.
- [Inlay hints](https://www.jetbrains.com/help/idea/inlay-hints.html)
  - Inlay hints are special markers that appear in the editor and provide you with additional information about your code, like the names of the parameters that a called method expects.
- [Color Provider](https://code.visualstudio.com/api/language-extensions/programmatic-language-features#show-color-decorators)
  - View all inlay colorful label for color literals in your document.
  - Change the color literal's value by a color picker or its code presentation.
- [Code Lens](https://code.visualstudio.com/blogs/2017/02/12/code-lens-roundup)
  - Should give contextual buttons along with code. For example, a button for exporting your document to various formats at the start of the document.
- [Rename symbols and embedded paths](https://code.visualstudio.com/api/language-extensions/programmatic-language-features#rename-symbols)
- [Help with function and method signatures](https://code.visualstudio.com/api/language-extensions/programmatic-language-features#help-with-function-and-method-signatures)
- [Workspace Symbols](https://code.visualstudio.com/api/language-extensions/programmatic-language-features#show-all-symbol-definitions-in-folder)
- [Code Action](https://learn.microsoft.com/en-us/dynamics365/business-central/dev-itpro/developer/devenv-code-actions)
  - Increasing/Decreasing heading levels.
  - Turn equation into "inline", "block" or "multiple-line block" styles.
- [experimental/onEnter](https://github.com/rust-lang/rust-analyzer/blob/master/docs/dev/lsp-extensions.md#on-enter)
  - <kbd>Enter</kbd> inside triple-slash comments automatically inserts `///`
  - <kbd>Enter</kbd> in the middle or after a trailing space in `//` inserts `//`
  - <kbd>Enter</kbd> inside `//!` doc comments automatically inserts `//!`
  - <kbd>Enter</kbd> inside equation markups automatically inserts indents.

Extra features:

- Compiles to PDF on save (configurable to as-you-type, or other options).
- Compiles to SVG, PNG, HTML, Markdown, Text, and other formats by commands, vscode tasks, or code lenses.
- Provides code lenses for exporting to PDF/SVG/PNG/etc.
- Provides a status bar item to show the current document's compilation status and words count.
- [Editor tools](https://github.com/Myriad-Dreamin/tinymist/tree/main/tools/editor-tools):
  - View a list of templates in template gallery. (`tinymist.showTemplateGallery`)
  - Click a button in template gallery to initialize a new project with a template. (`tinymist.initTemplate` and `tinymist.initTemplateInPlace`)
  - Trace execution in current document (`tinymist.profileCurrentFile`).


## Versioning and Release Cycle

Tinymist's versions follow the [Semantic Versioning](https://semver.org/) scheme, in format of `MAJOR.MINOR.PATCH`. Besides, tinymist follows special rules for the version number:
- If a version is suffixed with `-rcN` (<picture><source media="(prefers-color-scheme: dark)" srcset="./assets/images/introduction.typ-inlined0.svg"><img style="vertical-align: -0.35em" alt="typst-block" src="./assets/images/introduction.typ-inlined1.svg" /></picture>), e.g. `0.11.0-rc1` and `0.12.1-rc1`, it means this version is a release candidate. It is used to test publish script and E2E functionalities. These versions will not be published to the marketplace.
- If the `PATCH` number is odd, e.g. `0.11.1` and `0.12.3`, it means this version is a nightly release. The nightly release will use both [tinymist](https://github.com/Myriad-Dreamin/tinymist/tree/main) and [typst](https://github.com/typst/typst/tree/main) at **main branch**. They will be published as prerelease version to the marketplace.
- Otherwise, if the `PATCH` number is even, e.g. `0.11.0` and `0.12.2`, it means this version is a regular release. The regular release will always use the recent stable version of tinymist and typst.

The release cycle is as follows:
- If there is a typst version update, a new major or minor version will be released intermediately. This means tinymist will always align the minor version with typst.
- If there is at least a bug or feature added this week, a new patch version will be released.

## Installation

Follow the instructions to enable tinymist in your favorite editor.
- [VS Cod(e,ium)](https://myriad-dreamin.github.io/tinymist//frontend/vscode.html)
- [NeoVim](https://myriad-dreamin.github.io/tinymist//frontend/neovim.html)
- [Emacs](https://myriad-dreamin.github.io/tinymist//frontend/emacs.html)
- [Sublime Text](https://myriad-dreamin.github.io/tinymist//frontend/sublime-text.html)
- [Helix](https://myriad-dreamin.github.io/tinymist//frontend/helix.html)
- [Zed](https://myriad-dreamin.github.io/tinymist//frontend/zed.html)

## Installing Regular/Nightly Prebuilds from GitHub

Note: if you are not knowing what is a regular/nightly release, please don't follow this section.

Besides published releases specific for each editors, you can also download the latest regular/nightly prebuilts from GitHub and install them manually.

- Regular prebuilts can be found in [GitHub Releases](https://github.com/Myriad-Dreamin/tinymist/releases).
- Nightly prebuilts can be found in [GitHub Actions](https://github.com/Myriad-Dreamin/tinymist/actions). For example, if you are seeking a nightly release for the featured [PR: build: bump version to 0.11.17-rc1](https://github.com/Myriad-Dreamin/tinymist/pull/468), you could click and go to the [action page](https://github.com/Myriad-Dreamin/tinymist/actions/runs/10120639466) run for the related commits and download the artifacts.

To install extension file (the file with `.vsix` extension) manually, please <kbd>Ctrl+Shift+X</kbd> in the editor window and drop the downloaded vsix file into the opened extensions view.

## Documentation

See [Online Documentation](https://myriad-dreamin.github.io/tinymist/).

## Packaging

Stable Channel:

[![Packaging status](https://repology.org/badge/vertical-allrepos/tinymist.svg)](https://repology.org/project/tinymist/versions)

Nightly Channel:

[![Packaging status](https://repology.org/badge/vertical-allrepos/tinymist-nightly.svg)](https://repology.org/project/tinymist-nightly/versions)

## Roadmap

The development in typst v0.12.0 has been finished. We'll slow down for a while to catch regressions and bugs by changes. We are also planning to implement the following features in typst v0.13.0 or spare time in weekend:

- Spell checking: There is already a branch but no suitable (default) spell checking library is found.
- Type checking: complete the type checker.
- Static Linter: linting code statically according to feedback of the type checker and succeeding code analysis.
- Periscope renderer: It is disabled since vscode reject to render SVGs containing foreignObjects.
- Inlay hint: It is disabled _by default_ because of performance issues.
- Find references of dictionary fields and named function arguments.
- A reliable way of configuring projects's entry files and files to export across editors. See [GitHub Issue 530.](https://github.com/Myriad-Dreamin/tinymist/issues/530)
- Improve symbol view's appearance.
- Improve package view.
  - Navigate to symbols by clicking on the symbol name in the view.
  - Automatically locate the symbol item in the view when viewing local documentation.
  - Remember the recently invoked package commands, e.g. "Open Docs of \@preview/cetz:0.3.1", "Open directory of \@preview/touying:0.5.3".
- Improve label view.
  - Group labels.
  - Search labels.
  - Keep (persist) group preferences.
- Improve Typst Preview.
  - Browsing mode: if no main file is specified, the preview will be in browsing mode and use the recently focused file as the main.
  - Pin drop-down: Set the file to preview in the drop-down for clients that doesn't support passing arguments to the preview command.
  - Render in web worker (another thread) to reduce overhead on the electron's main thread.

If you are interested by any above features, please feel free to send Issues to discuss or PRs to implement to [GitHub.](https://github.com/Myriad-Dreamin/tinymist)

## Contributing

Please read the [CONTRIBUTING.md](CONTRIBUTING.md) file for contribution guidelines.

## Maintainers

Get list of maintainers from [MAINTAINERS.typ](https://github.com/Myriad-Dreamin/tinymist/blob/main/MAINTAINERS.typ). Or programmatically by `yarn maintainers`

> [!NOTE]
> 
>   You can add extra arguments for specific information. For example, `yarn maintainers --input="action=maintainers"`.


## Acknowledgements

- Partially code is inherited from [typst-lsp](https://github.com/nvarner/typst-lsp)
- The [integrating](https://github.com/Myriad-Dreamin/tinymist/tree/main/editors/vscode#symbol-view) **offline** handwritten-stroke recognizer is powered by [Detypify](https://detypify.quarticcat.com/).
- The [integrating](https://github.com/Myriad-Dreamin/tinymist/tree/main/editors/vscode#preview-command) preview service is powered by [typst-preview](https://github.com/Enter-tainer/typst-preview).
- The [integrating](https://github.com/Myriad-Dreamin/tinymist/tree/main/editors/vscode#managing-local-packages) local package management functions are adopted from [vscode-typst-sync](https://github.com/OrangeX4/vscode-typst-sync).
