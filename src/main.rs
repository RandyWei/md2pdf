use std::fs::File;
use std::io::{Read, Write};

use pulldown_cmark::{Event, Parser, Tag};

fn main() {

    let mut markdown_input = String::new();
    let mut input = File::open("markdown.md").unwrap();
    input.read_to_string(&mut markdown_input).unwrap();

    let parser = Parser::new(&markdown_input);

    let mut tex_output = String::new();
    tex_output.push_str("\\documentclass{scrartcl}\n");
    tex_output.push_str("\\usepackage{graphicx}\n");
    tex_output.push_str("\\usepackage{listings}\n");
    tex_output.push_str("\\usepackage{xcolor}\n");
    tex_output.push_str("\\definecolor{colKeys}{rgb}{0,0.5,0}\n");
    tex_output.push_str("\\definecolor{colIdentifier}{rgb}{0,0,0}\n");
    tex_output.push_str("\\definecolor{colComments}{rgb}{0,0.5,1}\n");
    tex_output.push_str("\\definecolor{colString}{rgb}{0.6,0.1,0.1}\n");
    tex_output.push_str("\\definecolor{colBackground}{rgb}{0.95,0.95,1}\n");
    tex_output.push_str("\\lstset{%configuration de listings\n");
    tex_output.push_str("    float=hbp,%\n");
    tex_output.push_str("        basicstyle=\\ttfamily\\small,%\n");
    tex_output.push_str("        %\n");
    tex_output.push_str("        identifierstyle=\\color{colIdentifier}, %\n");
    tex_output.push_str("        keywordstyle=\\color{colKeys}, %\n");
    tex_output.push_str("        stringstyle=\\color{colString}, %\n");
    tex_output.push_str("        commentstyle=\\color{colComments}\textit, %\n");
    tex_output.push_str("        %\n");
    tex_output.push_str("        backgroundcolor=\\color{colBackground},%\n");
    tex_output.push_str("        %\n");
    tex_output.push_str("        columns=flexible, %\n");
    tex_output.push_str("        tabsize=2, %\n");
    tex_output.push_str("        frame=trbl, %\n");
    tex_output.push_str("        %frameround=tttt,%\n");
    tex_output.push_str("        extendedchars=true, %\n");
    tex_output.push_str("        showspaces=false, %\n");
    tex_output.push_str("        showstringspaces=false, %\n");
    tex_output.push_str("        numbers=left, %\n");
    tex_output.push_str("        numberstyle=\\tiny, %\n");
    tex_output.push_str("        breaklines=true, %\n");
    tex_output.push_str("        breakautoindent=true, %\n");
    tex_output.push_str("        captionpos=b,%\n");
    tex_output.push_str("        xrightmargin=0.2cm, %\n");
    tex_output.push_str("        xleftmargin=0.2cm\n");
    tex_output.push_str("}\n");
    tex_output.push_str("\\begin{document}\n");

    for event in parser {
        match event {
            Event::Start(Tag::Header(level)) => {
                tex_output.push_str("\\");
                for _ in 1 .. level {
                    tex_output.push_str("sub");
                }
                tex_output.push_str("section{");
            },
            Event::End(Tag::Header(_)) => tex_output.push_str("}\n"),

            Event::Start(Tag::Emphasis) => tex_output.push_str("\\emph{"),
            Event::End(Tag::Emphasis) => tex_output.push_str("}"),

            Event::Start(Tag::Strong) => tex_output.push_str("\\textbf{"),
            Event::End(Tag::Strong) => tex_output.push_str("}"),

            Event::Start(Tag::List(None)) => tex_output.push_str("\\begin{itemize}\n"),
            Event::End(Tag::List(None)) => tex_output.push_str("\\end{itemize}\n"),

            Event::Start(Tag::List(Some(_))) => tex_output.push_str("\\begin{enumerate}\n"),
            Event::End(Tag::List(Some(_))) => tex_output.push_str("\\end{enumerate}\n"),

            Event::Start(Tag::Image(_, path, title)) => {
                tex_output.push_str("\\begin{figure}\n");
                tex_output.push_str("\\centering\n");
                tex_output.push_str("\\includegraphics[width=\\textwidth]{");;
                tex_output.push_str(&*path);
                tex_output.push_str("}\n");
                tex_output.push_str("\\caption{");
                tex_output.push_str(&*title);
                tex_output.push_str("}\n\\end{figure}\n");
            },

            Event::Start(Tag::Item) => tex_output.push_str("\\item "),
            Event::End(Tag::Item) => tex_output.push_str("\n"),

            Event::Start(Tag::CodeBlock(lang)) => {
                if ! lang.is_empty() {
                    tex_output.push_str("\\begin{lstlisting}[language=");
                    tex_output.push_str(&*lang);
                    tex_output.push_str("]\n");
                } else {
                    tex_output.push_str("\\begin{lstlisting}\n");
                }
            },

            Event::End(Tag::CodeBlock(_)) => {
                tex_output.push_str("\n\\end{lstlisting}\n");
            }

            Event::Text(t) => tex_output.push_str(&*t),
            _ => (),
        }
    }

    tex_output.push_str("\n\\end{document}\n");

    println!("{}", tex_output);

    let pdf_data = tectonic::latex_to_pdf(tex_output).expect("processing failed");
    let mut output = File::create("output.pdf").unwrap();
    output.write(&pdf_data).unwrap();
}
