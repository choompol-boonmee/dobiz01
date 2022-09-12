
use std::fmt::Write;

//==== TITLE
pub fn or2_title(s : &mut String) {
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
pub fn or2_toprow(s : &mut String) {
//	use std::io::Write;
	writeln!(s, 
r###"
<h2>หน่วยงาน กับ ใบอนุญาต/บริการ</h2>
<a href='/'>กลับหน้าหลัก</a>
<table border='1'>
<tr class='topbar'>
<td class='center'>{}</td>
<td class='center'>{}</td>
<td class='center'>{}</td>
<td class='center'>{}</td>
<td class='center'>{}</td>
<td class='center'>{}</td>
<td class='center'>{}</td>
<td class='center'>{}</td>
<td class='center'>{}</td>
<td class='center'>{}</td>
</tr>"###
, "ลำดับ"
, "หน่วยงาน"
, "จำนวน"
, "มีสถิติ"
, "มีระบบ"
, "แผน66"
, "แผน67"
, "แผน68"
, "เจตนา"
, "URI"
	).unwrap();
}

//==== EACH ROW
pub fn or2_onerow(s : &mut String, 
	no: &u32,
    nm: &str,
    cn: &u32,
	sta: &str,
	sys: &str,
	p66: &str,
	p67: &str,
	p68: &str,
	pro: &str,
	id: &u32
) {
//	use std::io::Write;
	writeln!(s , r###"
<tr>
<td>{}</td>
<td>{} </td>
<td class='right'>{}</td>
<td id='{}' class='data right'></td>
<td id='{}' class='data center'></td>
<td id='{}' class='data center'></td>
<td id='{}' class='data center'></td>
<td id='{}' class='data center'></td>
<td id='{}' class='data center'></td>
<td>or:{}</td>
</tr>
"###
, no, nm, cn
, sta, sys, p66, p67, p68, pro
, id
	).unwrap();
}

//==== END
pub fn or2_end(s : &mut String) {
	writeln!(s,
r###"
</table>
</body>
<script src="org2.js"></script>
</html>
"###
	).expect("ERR");
}

