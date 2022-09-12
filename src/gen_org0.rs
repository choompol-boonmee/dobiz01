
use std::fmt::Write;

//==== TITLE
pub fn or0_title(s : &mut String) {
//	use std::io::Write;
	writeln!(s,
r###"
<head>
<title>โครงการจ้างที่ปรึกษาวิจัยแนวทางการพัฒนาระบบอำนวยความสะดวกในการประกอบธุรกิจแบบครบวงจร</title>
<meta charset='UTF-8'>
<meta name='viewport' content='width=device-width, initial-scale=1.0'>
<link rel="stylesheet" href="style.css">
</head>
<body>
"###).unwrap();
}

//==== TOPROW
pub fn or0_toprow(s : &mut String) {
//	use std::io::Write;
	writeln!(s, 
r###"
<h2>หน่วยงาน กับ ใบอนุญาต/บริการ</h2>
<a href='/'>กลับหน้าหลัก</a>
<a href='orglist2.html'>ดูสถานะ</a>
<table border='1'>
<tr class='topbar'>
<td class='center'>{}</td>
<td class='center'>{}</td>
<td class='center'>{}</td>
<td class='center'>{}</td>
</tr>"###
, "ลำดับ", "หน่วยงาน", "จำนวน", "URI"
	).unwrap();
}

//==== EACH ROW
pub fn or0_onerow(s : &mut String, 
	no: &u32,
    nm: &str,
	ed: &str,
    cn: &u32,
	id: &u32) {
//	use std::io::Write;
	writeln!(s , r###"
<tr>
<td>{}</td>
<td>{} {} </td>
<td class='right'>{}</td>
<td>or:{}</td>
</tr>
"###
, no, nm, ed, cn, id
	).unwrap();
}

//==== END
pub fn or0_end(s : &mut String) {
	writeln!(s,
r###"
</table>
</body>
</html>
"###
	).expect("ERR");
}

