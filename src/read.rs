
use super::Service;

use calamine::{open_workbook, Error, Xlsx, Reader, DataType};
use std::collections::HashMap;

pub fn read2(fnm : &str, sht : &str)  -> Result<(Vec<(String,u32)>,HashMap<String, Vec<Service>>), Error> {
	let mut memb : HashMap<String, Vec<Service>> = HashMap::new();
	let mut ord : Vec<(String,u32)> = vec![];
	let mut cnt = HashMap::new();
	let mut workbook: Xlsx<_> = open_workbook(fnm)?;
	if let Some(Ok(r)) = workbook.worksheet_range(sht) {
//println!("read sheet {}", &sht);
		for r in r.rows() {
//			let (mut org, mut srv, mut no, mut gg1, mut gg2) 
//			  = (String::new(), String::new(), 0, 0, 0);
			let mut svif = Service { 
				org : String::new(), 
				srv : String::new(), 
				no : 0u32,
				gg1 : 0u32,
				gg2 : 0u32,
				sco : 0u32,
			};
			for (i, c) in r.iter().enumerate() {
				match i {
					0 => if let DataType::Float(ref n) = c { svif.no = *n as u32; }
					2 => if let DataType::String(ref s) = c { svif.org = String::from(s); },
					3 => if let DataType::String(ref s) = c { svif.srv = String::from(s); },
					37 => if let DataType::Float(ref f) = c { svif.gg1 = *f as u32; }
					38 => if let DataType::Float(ref f) = c { svif.gg2 = *f as u32; }
					59 => if let DataType::Float(ref f) = c { svif.sco = *f as u32; }
					_ => {}
				}
			}
			if svif.no==0 || svif.org=="" || svif.srv=="" { continue; }
//if svif.no==3452 { println!("no:{} {} {}", svif.no, svif.org, svif.srv); }
			cnt.insert(svif.org.clone(), if cnt.contains_key(&svif.org) { cnt[&svif.org]+1 } else { 1u32 });
			let _cc = cnt[&svif.org].clone();
			if let Some(mb) = memb.get_mut(&svif.org) {
				mb.push(svif.clone());
			} else {
				memb.insert(svif.org.clone(), vec![svif]);
			}
		}
		
		for (_k,v) in &mut memb {
//println!(" MEMB: {} {}", 
			v.sort_by(|a,b| b.gg1.cmp(&a.gg1));
		}

		ord = cnt.into_iter().map(|(id,cc)| (String::from(id),cc)).collect();
		ord.sort_by(|a,b| b.1.cmp(&a.1));
	}
	Ok((ord,memb))
}

pub fn read_org2(fnm : &str, sht : &str)  -> Result<Vec<(u32,String)>, Error> {
	let mut org = vec![];
	let mut workbook: Xlsx<_> = open_workbook(fnm)?;
	if let Some(Ok(r)) = workbook.worksheet_range(sht) {
		for r in r.rows() {
			let (mut id, mut name) = (0u32, String::new());
			for (i, c) in r.iter().enumerate() {
				match i {
					0 => if let DataType::Float(ref n) = c { id = *n as u32; },
					1 => if let DataType::String(ref n) = c { name = String::from(n); },
					_ => {},
				}
			}
			if id==0 || name=="" { continue }
			org.push((id, name.clone()));
		}
	}
	Ok(org)
}

pub fn rowread(fnm : &str, sht : &str)  -> Result<(), Error> {
	let mut workbook: Xlsx<_> = open_workbook(fnm)?;
	if let Some(Ok(r)) = workbook.worksheet_range(&sht) {
		let mut cc = 0;
		let mut r2 = vec![];
		let mut r4 = vec![];
		for r in r.rows() {
			cc += 1;
			if cc!=2 && cc!=4  { continue; }
			let mut flds = vec![];
			for (_i, c) in r.iter().enumerate() { flds.push(c); }
			if cc==2  { r2 = flds; } else if cc==4  { r4 = flds; break; }
		}
		for i in 0..r2.len() {
			println!("{}: {}: {:?}", i, r2[i], r4[i]);
		}
	}
	Ok(())
}


