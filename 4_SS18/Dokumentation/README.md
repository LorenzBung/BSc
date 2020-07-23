# doku-pss
This repository contains the documentation of my practical semestre, which I absolved in SS18.
Also, a template for other latex projects can be found on the branch `template`.

## Content of this project

### Folders
- `format/`: Everything needed to setup the document style.
  This includes packages, configuration, etc.
- `media/`: All media like images etc.
- `content/`: The actual content of the document.

### Files
- `main.tex`: The main LaTeX-file. Here, an abstract and some style setup can be found.
- `format/packages.tex`: A file to include all important packages.
- `format/code.tex`: A file to define a good-looking source code environment using the package `lstlistings`.

## Using this project as a template
If you like the setup of this project, feel free to re-use it.
In the `template`-branch you can find the structure and setup of the latex files, without any content.

If you need to create a large document, such as a bachelor thesis, **split up the content**.
You can do that by e.g. creating a folder `content/`, and then putting a file `chapterN.tex` for each
chapter there (or even for each section, subsection etc).
In the main file, only put a `\input{content/chapterN}` to use it then (see my documentation for reference).
This will keep everything structured and help a lot with readability.
