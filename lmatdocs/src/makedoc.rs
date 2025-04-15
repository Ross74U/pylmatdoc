use super::parser;
use docx_rs::alignment_type::*;
use docx_rs::*;

use std::io::Cursor;

pub fn makedoc(tree: &Vec<parser::Paragraph>) -> Result<Vec<u8>, DocxError> {
    /* bullet points */
    let bullet_numbering = AbstractNumbering::new(1)
        .add_level(
            Level::new(
                1,
                Start::new(1),
                NumberFormat::new("bullet"),
                LevelText::new("●"),
                LevelJc::new("left"),
            )
            .indent(Some(540), Some(SpecialIndentType::Hanging(180)), None, None),
        )
        .add_level(
            Level::new(
                2,
                Start::new(1),
                NumberFormat::new("bullet"),
                LevelText::new("○"),
                LevelJc::new("left"),
            )
            .indent(
                Some(1080),
                Some(SpecialIndentType::Hanging(180)),
                None,
                None,
            ),
        );

    let mut doc = Docx::new().add_abstract_numbering(bullet_numbering);

    for node in tree {
        let mut p = Paragraph::new();
        match node.delim {
            parser::Delimiter::Centered => p = p.align(AlignmentType::Center),
            parser::Delimiter::Bullet => {
                p = p.numbering(NumberingId::new(1), IndentLevel::new(1));
            }
            parser::Delimiter::SubBullet => {
                p = p.numbering(NumberingId::new(1), IndentLevel::new(2));
            }
            _ => {}
        }

        for r in node.runs.iter() {
            let mut run = Run::new().add_text(r.text).size(24);
            if r.format.bold {
                run = run.bold();
            }
            if r.format.italic {
                run = run.italic();
            }
            if r.format.underline {
                run = run.underline("single");
            }
            p = p.add_run(run);
        }
        doc = doc.add_paragraph(p);
    }

    let mut buffer = Cursor::new(Vec::new());
    doc.build().pack(&mut buffer)?;
    Ok(buffer.into_inner())
}
