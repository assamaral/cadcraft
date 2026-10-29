//! A centre-justified ATTDEF keeps its insertion point in a DWG (#2464): AutoCAD puts it where the
//! baseline of the displayed tag starts, which for an attribute with an empty default value is
//! further from the alignment point than half a height.

use cadcraft_doc::EntityKind;
use cadcraft_io::{SaveOptions, Version, read, write_with};

/// The `insert` x of the only ATTDEF in `d`.
fn attdef_x(d: &cadcraft_doc::Drawing) -> f64 {
    d.model.iter().find_map(|e| if let EntityKind::AttDef(a) = &e.kind { Some(a.text.insert.x) } else { None }).expect("an ATTDEF")
}

#[test]
fn centred_attdef_with_an_empty_value_keeps_its_insertion_point() {
    // The tag "LONGTAG" at height 2.5, centred on x = 100: its baseline starts half a width left.
    let width = cadcraft_fonts::line_width("LONGTAG", 2.5, 1.0);
    let start = 100.0 - width / 2.0;
    assert!(100.0 - start > 3.0, "the point is well over a height from the alignment point");
    let dxf = format!(
        "0\nSECTION\n2\nENTITIES\n0\nATTDEF\n8\n0\n10\n{start}\n20\n5.0\n40\n2.5\n1\n\n7\nSTANDARD\n72\n1\n11\n100.0\n21\n5.0\n3\nprompt\n2\nLONGTAG\n70\n8\n0\nENDSEC\n0\nEOF\n"
    );
    let d = read(dxf.as_bytes(), "t.dxf").unwrap();
    assert!((attdef_x(&d) - start).abs() < 1e-6, "read from DXF");
    for version in [Version::R2000, Version::R2018] {
        let dwg = write_with(&d, "t.dwg", &SaveOptions { version: Some(version), binary: false }).unwrap();
        let back = read(&dwg, "t.dwg").unwrap();
        assert!((attdef_x(&back) - start).abs() < 1e-6, "{version:?}: insertion point {} instead of {start}", attdef_x(&back));
    }
}
