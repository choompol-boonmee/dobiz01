
use std::fmt::Write;
use super::Service;

//====== SERVICE0 HEAD
pub fn srv0_head(ss : &mut String, nm : &str, id : &u32) {
	writeln!(ss,
r###"
<head>
<title>โครงการจ้างที่ปรึกษาวิจัยแนวทางการพัฒนาระบบอำนวยความสะดวกในการประกอบธุรกิจแบบครบวงจร</title>
<meta charset='UTF-8'>
<meta name='viewport' content='width=device-width, initial-scale=1.0'>
<link rel="stylesheet" href="style.css">
<script>
let orgid = 'or-{}';
</script>
</head>
<body>
"###, &id).unwrap();
	writeln!(ss, 
r###"
<h2>ใบอนุญาต/บริการของ {} (**)</h2>
<span id='orgid'>(or-{})</span> <span id='reload' class=''>R</span>
<a href='orglist.html'>กลับหน้าหน่วยงาน</a>
"###, nm, id).unwrap();
	writeln!(ss, "<table border='1'>").unwrap();
	writeln!(ss, 
r###"
<tr class='topbar'>
<td rowspan=2>
ลำดับ
</td>
<td class='center' rowspan=2>
ชื่อใบอนุญาต/บริการ<br>

<!-- span id='sv_name' class='btn'>ED</span>
<span id='sv_name_all' class='btn'>✓</span>
<dialog id="sv_name_dlg">
<div>ชื่อใบอนุญาต</div>
<input id="sv_name_val" ><br>
<span id='sv_name_run' class='runbtn'>แก้ไข</span>
<span id='sv_name_can' class='canbtn'>ยกเลิก</span>
</dialog -->

</td>

<td class='center' colspan=2>ปริมาณ</td>

<td class='center' rowspan=2>
มี<br>ระบบ<br>
<span id='sv_elec' class='btn'>ED</span>
<span id='sv_elec_all' class='btn'>✓</span>
<dialog id="sv_elec_dlg">
<div>มีระบบหรือไม่</hdiv>
<input id="sv_elec_val" > (Y:มี) <br>
<span id='sv_elec_run' class='runbtn'>แก้ไข</span>
<span id='sv_elec_can' class='canbtn'>ยกเลิก</span>
</dialog>
</td>

<td class='center' colspan=3>แผนพัฒนาระบบ</td>
</td>

<td class='center' rowspan=2>
SCO
</td>

<td class='center' rowspan=2>
URI<br>
IRI
</td>

<td class='center' rowspan=2>
PRO<br>
MOT<br>
<span id='sv_pro' class='btn'>ED</span>
<span id='sv_pro_all' class='btn'>✓</span>
<dialog id="sv_pro_dlg">
<div>ระดับความสำคัญ เพื่อนำร่อง</hdiv>
<input id="sv_pro_val" > (1-5: 5:มากที่สุด)<br>
<span id='sv_pro_run' class='runbtn'>แก้ไข</span>
<span id='sv_pro_can' class='canbtn'>ยกเลิก</span>
</dialog>
</td>

</tr>

<tr class='topbar'>

<td class='center'>ครั้ง<br>
<span id='sv_count' class='btn'>ED</span>
<span id='sv_count_all' class='btn'>✓</span>
<dialog id="sv_count_dlg">
<div>จำนวนครั้ง</hdiv>
<input id="sv_count_val" ><br>
<span id='sv_count_run' class='runbtn'>แก้ไข</span>
<span id='sv_count_can' class='canbtn'>ยกเลิก</span>
</dialog>
</td>

<td class='center'>ต่อ<br>
<span id='sv_per' class='btn'>ED</span>
<span id='sv_per_all' class='btn'>✓</span>
<dialog id="sv_per_dlg">
<div>ต่อระยะเวลา</div>
<input id="sv_per_val" > (ต่อเดือน,ต่อปี)<br>
<span id='sv_per_run' class='runbtn'>แก้ไข</span>
<span id='sv_per_can' class='canbtn'>ยกเลิก</span>
</dialog>
</td>

<td class='center'>แผน66<br>
<span id='sv_p66' class='btn'>ED</span>
<span id='sv_p66_all' class='btn'>✓</span>
<dialog id="sv_p66_dlg">
<div>แผนปี2566</hdiv>
<input id="sv_p66_val" > (Y:มีแผน)<br>
<span id='sv_p66_run' class='runbtn'>แก้ไข</span>
<span id='sv_p66_can' class='canbtn'>ยกเลิก</span>
</dialog>
</td>

<td class='center'>แผน67<br>
<span id='sv_p67' class='btn'>ED</span>
<span id='sv_p67_all' class='btn'>✓</span>
<dialog id="sv_p67_dlg">
<div>แผนปี2567</hdiv>
<input id="sv_p67_val" > (Y:มีแผน)<br>
<span id='sv_p67_run' class='runbtn'>แก้ไข</span>
<span id='sv_p67_can' class='canbtn'>ยกเลิก</span>
</dialog>
</td>

<td class='center'>แผน68<br>
<span id='sv_p68' class='btn'>ED</span>
<span id='sv_p68_all' class='btn'>✓</span>
<dialog id="sv_p68_dlg">
<div>แผนปี2568</hdiv>
<input id="sv_p68_val" > (Y:มีแผน)<br>
<span id='sv_p68_run' class='runbtn'>แก้ไข</span>
<span id='sv_p68_can' class='canbtn'>ยกเลิก</span>
</dialog>
</td>

</tr>
"###
	).unwrap();
}

//====== SERVICE0 ONEROW
pub fn srv0_onerow(ss : &mut String, v : &Service, cc : &i32, sco_cls : &str) {
	writeln!(ss, 
r###"<tr><td class='center'>{}</td>
<td id='sv-{}_name' class=''>{}</td>
<td id='sv-{}_count' class='edit right'></td>
<td id='sv-{}_per' class='edit center'>เดือน</td>
<td id='sv-{}_elec' class='edit center'></td>
<td id='sv-{}_p66' class='edit center'></td>
<td id='sv-{}_p67' class='edit center'></td>
<td id='sv-{}_p68' class='edit center'></td>
<td class='center {}'>{}</td>
<td class='center'>{}</td>
<td id='sv-{}_pro' class='edit center'></td>
</tr>"### , &cc
, v.no, v.srv
, v.no, v.no, v.no, v.no, v.no, v.no
, sco_cls, v.sco, v.no, v.no
	).unwrap();
}

//====== SERVICE0 TAIL
pub fn srv0_tail(ss : &mut String) {
	writeln!(ss,
r###"
</table>
<a href='orglist.html'>กลับหน้าหน่วยงาน</a>
</body>
<script src="service.js"></script>
</html>
"###
	).unwrap();
}

