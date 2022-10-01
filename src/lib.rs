use pulldown_cmark::{Event, Parser, Tag};

pub const LATEX_HEADER:&str = r#"\documentclass{scrartcl}
\usepackage{graphicx}
\usepackage{hyperref}
\usepackage{listings}
\usepackage{xcolor}
\definecolor{colKeys}{rgb}{0,0.5,0}
\definecolor{colIdentifier}{rgb}{0,0,0}
\definecolor{colComments}{rgb}{0,0.5,1}
\definecolor{colString}{rgb}{0.6,0.1,0.1}
\definecolor{colBackground}{rgb}{0.95,0.95,1}
\lstset{%configuration de listings
   float=hbp,%
   basicstyle=\ttfamily\small,%
   %
   identifierstyle=\color{colIdentifier}, %
   keywordstyle=\color{colKeys}, %
   stringstyle=\color{colString}, %
   commentstyle=\color{colComments}\textit, %
   %
   backgroundcolor=\color{colBackground},%
   %
   columns=flexible, %
   tabsize=2, %
   frame=trbl, %
   %frameround=tttt,%
   extendedchars=true, %
   showspaces=false, %
   showstringspaces=false, %
   numbers=left, %
   numberstyle=\tiny, %
   breaklines=true, %
   breakautoindent=true, %
   captionpos=b,%
   xrightmargin=0.2cm, %
   xleftmargin=0.2cm
}
\begin{document}
"#;

pub const LATEX_FOOTER: &str = "\n\\end{document}\n";

pub fn markdown_to_latex(markdown: String) -> String {
    let mut output = String::from(LATEX_HEADER);

    let parser = Parser::new(&markdown);

    for event in parser {
        match event {
            Event::Start(Tag::Header(level)) => {
                output.push_str("\\");
                for _ in 1 .. level {
                    output.push_str("sub");
                }
                output.push_str("section{");
            },
            Event::End(Tag::Header(_)) => output.push_str("}\n"),

            Event::Start(Tag::Emphasis) => output.push_str("\\emph{"),
            Event::End(Tag::Emphasis) => output.push_str("}"),

            Event::Start(Tag::Strong) => output.push_str("\\textbf{"),
            Event::End(Tag::Strong) => output.push_str("}"),

            Event::Start(Tag::List(None)) => output.push_str("\\begin{itemize}\n"),
            Event::End(Tag::List(None)) => output.push_str("\\end{itemize}\n"),

            Event::Start(Tag::List(Some(_))) => output.push_str("\\begin{enumerate}\n"),
            Event::End(Tag::List(Some(_))) => output.push_str("\\end{enumerate}\n"),

            Event::Start(Tag::Link(_, url, _)) => {
                output.push_str("\\href{");
                output.push_str(&*url);
                output.push_str("}{");
            },

            Event::End(Tag::Link(_, _, _)) => {
                output.push_str("}");
            },

            Event::Start(Tag::Image(_, path, title)) => {
                output.push_str("\\begin{figure}\n");
                output.push_str("\\centering\n");
                output.push_str("\\includegraphics[width=\\textwidth]{");
                output.push_str(&*path);
                output.push_str("}\n");
                output.push_str("\\caption{");
                output.push_str(&*title);
                output.push_str("}\n\\end{figure}\n");
            },

            Event::Start(Tag::Item) => output.push_str("\\item "),
            Event::End(Tag::Item) => output.push_str("\n"),

            Event::Start(Tag::CodeBlock(lang)) => {
                if ! lang.is_empty() {
                    output.push_str("\\begin{lstlisting}[language=");
                    output.push_str(&*lang);
                    output.push_str("]\n");
                } else {
                    output.push_str("\\begin{lstlisting}\n");
                }
            },

            Event::End(Tag::CodeBlock(_)) => {
                output.push_str("\n\\end{lstlisting}\n");
            },

            Event::Text(t) => {
                output.push_str(&*t);
            },

            Event::SoftBreak => {
                output.push('\n');
            },

            _ => (),
        }
    }

    output.push_str(LATEX_FOOTER);

    output
}

pub fn markdown_to_pdf(markdown: String) -> Result<Vec<u8>, tectonic::Error> {
    tectonic::latex_to_pdf(markdown_to_latex(markdown))
}

