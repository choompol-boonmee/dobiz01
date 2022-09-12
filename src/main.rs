
pub mod prog1;
pub mod read;
pub mod gen_org0;
pub mod gen_org2;
pub mod gen_srv0;

use std::env::{args};
use calamine::{open_workbook, Error, Xlsx, Reader, DataType};
use std::collections::HashMap;
use rusttype::{Font, Scale};
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_text_mut, text_size, draw_filled_rect_mut,draw_line_segment_mut};
use imageproc::rect::Rect;
use std::fs;
use prog1::Service;
use read::{read2,read_org2,rowread};
use std::fmt::Write;
use std::path::Path;
use std::fs::File;
use gen_org0::*;
use gen_org2::*;
use gen_srv0::*;
//use std::io::Write;

fn main() {
	let dd = String::from("../../dga/pp2/e-license-20220825.xlsx");
	let a1 = args().nth(1).unwrap_or("none".to_string());
	match a1.as_str() {
		"read" => {
			let f=args().nth(2).unwrap_or(dd);
			let _ = read(&f);
		},
		"rowread" => {
			let f=args().nth(2).unwrap_or(dd);
			let _ = rowread(&f, "sv1-บริการทั้งหมด");
		},
		"org" => {
			let f=args().nth(2).unwrap_or(dd);
			if let Ok(rs) = read_org(&f, "or1-หน่วยงาน") {
				for v in &rs {
					println!("{} {} {}", v.0, v.1, v.2);
				}
			}
		},
		"row" => {
			let f = args().nth(2).unwrap_or(dd);
			let r = args().nth(3).unwrap_or("2".to_string());
			let r = r.parse::<u32>().unwrap();
			let _ = row(&f, r);
		},
		"ana1" => {
			let f=args().nth(2).unwrap_or(dd);
			if let Ok(rs) = ana1(&f) {
				let (ls,_mmb) = rs;
				for (s,c) in ls {
					println!("{} = {}", &s, &c);
				}
			}
		},
		"ana2" => {
			let f=args().nth(2).unwrap_or(dd);
			if let Ok(rs) = ana1(&f) {
				let (ls,mmb) = rs;
				let _ = proc1(ls, mmb);
			}
		},
		//-- turn text into graphics
		"ana3" => {
			let f=args().nth(2).unwrap_or(dd);
			if let Ok(or) = read_org(&f, "or1-หน่วยงาน") {
				if let Ok(rs) = ana1(&f) {
					let (ls,mmb) = rs;
					let mut hm : HashMap<String,u32> = HashMap::new();
					for o in &or { hm.insert(o.1.clone(), o.0); }
					for v in &ls {
						if let Some(_ii) = hm.get(&v.0) {
						} else {
							println!("NG == {}", v.0);
						}
					}
					let _ = prog1::proc2(&ls, &mmb, &hm, 800);
				}
			}
		},
		// get RDF for org and service
		"ana4" => {
			let f=args().nth(2).unwrap_or(dd);
			let out = "work/dobizdoc3";
			gen_service(& f[..], &out[..]);
		},
		_ => {},
	}
}

pub fn gen_service(f : &str, out : &str) {

		fs::create_dir_all(out).unwrap();
		let rdfdir = "work/dobizdoc/rdf";
		fs::create_dir_all(rdfdir).unwrap();

		let or = read_org(&f, "or1-หน่วยงาน");
		let se = read_org2(&f, "หน่วยเลือก");
		let rs = read2(&f, "sv1-บริการทั้งหมด");

		use std::io::Write;
		let mut s = String::new();
		let mut or1 = String::new();
		or0_title(&mut s);
		or2_title(&mut or1);
		if let (Ok(or),Ok(se),Ok(rs)) = (or,se,rs) {
			or0_toprow(&mut s);
			or2_toprow(&mut or1);
			let mut hm : HashMap<String,u32> = HashMap::new();
			for o in &or { hm.insert(o.1.clone(), o.0); }
			let mut ors = String::from(r###"
@prefix or: <http://dga.tueng.org/rdf/or#> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix org:        <http://www.w3.org/ns/org#> .
@prefix rdfs:       <http://www.w3.org/2000/01/rdf-schema#> .
"###);
			for o in &or {
write!(&mut ors, r###"
or:{}  a  org:Organization ;
rdfs:label '{}' .
"###
, o.0.clone(), o.1.clone()
).unwrap();
			}
//println!("{}", ors);
			let orgf = format!("{rdfdir}/org.ttl");
			let orgp = Path::new(&orgf);
			let mut orgf = File::create(orgp).expect("Unable to create file");
			orgf.write_all(ors.as_bytes()).expect("Unable to write data");

			let (_ls,mmb) = rs;
			for v in &se {
				if let Some(srv) = mmb.get(&v.1) {
					let id = hm.get(&v.1).expect("");
					let sfnm = format!("sv-{:?}.html", id);
					let lnk = format!("<a href='{}'>{}</a>", sfnm, &v.1);
					let edt = format!("<a href='editreq/or-{}'>[ขอแก้]</a>", &id);
					let sta = format!("or-{}-count", &id);
					let sys = format!("or-{}-elec", &id);
					let p66 = format!("or-{}-p66", &id);
					let p67 = format!("or-{}-p67", &id);
					let p68 = format!("or-{}-p68", &id);
					let pro = format!("or-{}-pro", &id);
					let cn0 = srv.len() as u32;
					or0_onerow(&mut s, &v.0, &lnk, &edt, &cn0, &id);
					or2_onerow(&mut or1, &v.0, &lnk, &cn0
						, &sta, &sys, &p66, &p67, &p68, &pro, &id);
//					let mut ss = String::new();
					let mut sv0 = String::new();
					let mut cc = 0;

					srv0_head(&mut sv0, &v.1, &id);

					for v in srv {
						cc += 1;
						let sco_cls = if v.sco>=8 { "high" } else { "" };

						srv0_onerow(&mut sv0, &v, &cc, &sco_cls);
					}
					srv0_tail(&mut sv0);

					let fp = format!("{out}/{sfnm}");
					let pt = Path::new(&fp);
					let mut f = File::create(pt).expect("Unable to create file");
					f.write_all(sv0.as_bytes()).expect("Unable to write data");

/*
					let fsrv0 = format!("{out}/{sfnm0}");
					let psrv0 = Path::new(&fsrv0);
					let mut f0 = File::create(psrv0).expect("Unable to create file");
					f0.write_all(sv0.as_bytes()).expect("Unable to write data");
*/

				}
			}
			or0_end(&mut s);
			or2_end(&mut or1);
		}
		let fp = format!("{out}/orglist.html");
		let n_lb = Path::new(&fp);
		let mut f = File::create(n_lb).expect("Unable to create file");
		f.write_all(s.as_bytes()).expect("Unable to write data");

		let orgfn2 = format!("{out}/orglist2.html");
		let orgpt2 = Path::new(&orgfn2);
		let mut orgf2 = File::create(orgpt2).expect("Unable to create file");
		orgf2.write_all(or1.as_bytes()).expect("Unable to write data");

}

pub fn proc1(ls: Vec<(String,u32)>, mmb: HashMap<String, Vec<Service>>) -> Result<(), Error> {
	let font = Vec::from(include_bytes!("THSarabunNew.ttf") as &[u8]);
	let font = Font::try_from_vec(font).unwrap();
	let scale = Scale { x: 80.0, y: 80.0 };
	let wht = Rgb([255u8,255u8,255u8]);
	let grn = Rgb([0u8,130u8,0u8]);
	let pic_fld = "work/pic1";
	fs::create_dir_all(pic_fld).unwrap();
	let mg = 20i32;
	for (o,_n) in ls {
		let srv = mmb.get(&o);
		if let Some(srv) = srv {
			for s in srv {
				
				let oid = s.no;

				//==== org name BEGIN
				let (w, h) = text_size(scale, &font, &o);
				let wd :u32 = (w + mg * 2).try_into().unwrap();
				let hg :u32 = (h + mg * 2).try_into().unwrap();
				let wdf = wd as f32;
				let hgf = hg as f32;
				let mut image = RgbImage::new(wd, hg);
				draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(wd, hg), grn);
				draw_text_mut(&mut image, wht, mg, mg, scale, &font, &o);
				for i in 0..2 {
					let ff : f32 = i as f32;
					draw_line_segment_mut(&mut image, (0.0,0.0+ff), (wdf,0.0+ff), wht);
					draw_line_segment_mut(&mut image, (0.0,hgf-1.0-ff), (wdf-1.0,hgf-1.0-ff), wht);
					draw_line_segment_mut(&mut image, (0.0+ff,0.0), (0.0+ff,hgf-1.0), wht);
					draw_line_segment_mut(&mut image, (wdf-1.0-ff,0.0), (wdf-1.0-ff,hgf-1.0), wht);
				}
				let path = format!("{pic_fld}/o_{oid}_nm.png");
				image.save(path).unwrap();
				//==== org name END

				//==== service name BEGIN
				let (w, h) = text_size(scale, &font, &s.srv);
				let wd :u32 = (w + mg * 2).try_into().unwrap();
				let hg :u32 = (h + mg * 2).try_into().unwrap();
				let wdf = wd as f32;
				let hgf = hg as f32;
				let mut image = RgbImage::new(wd, hg);
				draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(wd, hg), grn);
				draw_text_mut(&mut image, wht, mg, mg, scale, &font, &s.srv);
				for i in 0..2 {
					let ff : f32 = i as f32;
					draw_line_segment_mut(&mut image, (0.0,0.0+ff), (wdf,0.0+ff), wht);
					draw_line_segment_mut(&mut image, (0.0,hgf-1.0-ff), (wdf-1.0,hgf-1.0-ff), wht);
					draw_line_segment_mut(&mut image, (0.0+ff,0.0), (0.0+ff,hgf-1.0), wht);
					draw_line_segment_mut(&mut image, (wdf-1.0-ff,0.0), (wdf-1.0-ff,hgf-1.0), wht);
				}
				let path = format!("{pic_fld}/s_{oid}_0.png");
				image.save(path).unwrap();
				//==== service name END

				println!("{}-{}-{}", s.srv, s.org, s.no);
				break;
			} // for s
		} // Some(srv)
	}
	Ok(())
}

pub fn row(fnm : &str, tg : u32)  -> Result<(), Error> {
	let mut workbook: Xlsx<_> = open_workbook(fnm)?;
	if let Some(Ok(r)) = workbook.worksheet_range("คัดเลือก") {
		let mut rr = 0;
		for r in r.rows() {
			rr += 1;
			if tg!=rr { continue; };
			for (i, c) in r.iter().enumerate() {
				print!("{}: {:?}. ",i,c,);
			}
			println!("");
		}
//		let sz = r.get_size();
//		println!("all sz:{:?}", sz);
	}
	Ok(())
}

pub fn read(fnm : &str)  -> Result<(), Error> {
	let mut workbook: Xlsx<_> = open_workbook(fnm)?;
	if let Some(Ok(r)) = workbook.worksheet_range("คัดเลือก") {
		for r in r.rows() {
			for (i, c) in r.iter().enumerate() {
				print!("{}_{:?} ",i,c,);
			}
			println!("");
		}
//		let sz = r.get_size();
//		println!("all sz:{:?}", sz);
	}
	Ok(())
}

pub fn read_org(fnm : &str, sht : &str)  -> Result<Vec<(u32,String,String)>, Error> {
	let mut org = vec![];
	let mut workbook: Xlsx<_> = open_workbook(fnm)?;
	if let Some(Ok(r)) = workbook.worksheet_range(sht) {
		for r in r.rows() {
			let (mut id, mut name, mut up) = (0u32, String::new(), String::new());
			for (i, c) in r.iter().enumerate() {
				match i {
					0 => if let DataType::Float(ref n) = c { id = *n as u32; },
					1 => if let DataType::String(ref n) = c { name = String::from(n); },
					2 => if let DataType::String(ref n) = c { up = String::from(n); },
					_ => {},
				}
				if id==0 || name=="" || up=="" { continue }
				org.push((id, name.clone(), up.clone()));
			}
		}
	}
	Ok(org)
}

pub fn ana1(fnm : &str)  -> Result<(Vec<(String,u32)>,HashMap<String, Vec<Service>>), Error> {
	let mut memb : HashMap<String, Vec<Service>> = HashMap::new();
	let mut ord : Vec<(String,u32)> = vec![];
	let mut cnt = HashMap::new();
	let mut workbook: Xlsx<_> = open_workbook(fnm)?;
	if let Some(Ok(r)) = workbook.worksheet_range("คัดเลือก") {
		for r in r.rows() {
			let (mut org, mut srv, mut no, mut gg1, mut gg2, mut sco) 
			  = (String::new(), String::new(), 0, 0, 0, 0);
			for (i, c) in r.iter().enumerate() {
				match i {
					0 => if let DataType::Float(ref n) = c { no = *n as u32; }
					2 => if let DataType::String(ref s) = c { org = String::from(s); },
					3 => if let DataType::String(ref s) = c { srv = String::from(s); },
					37 => if let DataType::Float(ref f) = c { gg1 = *f as u32; }
					38 => if let DataType::Float(ref f) = c { gg2 = *f as u32; }
					59 => if let DataType::Float(ref f) = c { sco = *f as u32; }
					_ => {}
				}
			}
			if no==0 || org=="" || srv=="" { continue; }
			cnt.insert(org.clone(), if cnt.contains_key(&org) { cnt[&org]+1 } else { 1u32 });
			let sv = Service { 
				org : String::from(&org), 
				srv : String::from(&srv), 
				no : no,
				gg1 : gg1,
				gg2 : gg2,
				sco : sco,
			};
			if let Some(mb) = memb.get_mut(&org) {
				mb.push(sv);
			} else {
				memb.insert(org.clone(), vec![sv]);
			}
		}
		
		for (_k,v) in &mut memb {
			v.sort_by(|a,b| b.gg1.cmp(&a.gg1));
		}

		ord = cnt.into_iter().map(|(id,cc)| (String::from(id),cc)).collect();
		ord.sort_by(|a,b| b.1.cmp(&a.1));
	}
	Ok((ord,memb))
}

