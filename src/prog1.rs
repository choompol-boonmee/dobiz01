
use calamine::{ Error };
use std::collections::HashMap;
use rusttype::{Font, Scale};
use image::{Rgb, RgbImage, imageops};
use imageproc::drawing::{draw_text_mut, text_size, draw_filled_rect_mut,draw_line_segment_mut};
use imageproc::rect::Rect;
use std::fs;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct Service {
	pub org : String,
	pub srv : String,
	pub no : u32,
	pub gg1 : u32,
	pub gg2 : u32,
	pub sco : u32,
}

pub fn proc2(ls: &Vec<(String,u32)>
		, mmb: &HashMap<String, Vec<Service>>
		, hm: &HashMap<String,u32>
		, maxwd: u32
	) -> Result<(), Error> {

	let font = Vec::from(include_bytes!("THSarabunNew.ttf") as &[u8]);
	let font = Font::try_from_vec(font).unwrap();
	let scale = Scale { x: 80.0, y: 80.0 };
	let small = Scale { x: 30.0, y: 30.0 };
	let wht = Rgb([255u8,255u8,255u8]);
	let grn = Rgb([0u8,130u8,0u8]);
//	let red = Rgb([130u8,0u8,0u8]);
	let pic_fld = "work/pic2";
	fs::create_dir_all(pic_fld).unwrap();
	let mx = 70i32;
	let my = 20i32;
	for (o,_n) in ls {
		let or = o.clone();
		let srv = mmb.get(&or);
		if let Some(srv) = srv {
			for s in srv {
				
				let sid = s.no;

				//==== org name BEGIN
				let mut oid = 0u32;
				if let Some(ii) = hm.get(&s.org) { oid = ii.clone(); }
				let (w, h) = text_size(scale, &font, &o);
				let wd :u32 = (w + mx * 2).try_into().unwrap();
				let hg :u32 = (h + my * 2).try_into().unwrap();
				let wdf = wd as f32;
				let hgf = hg as f32;
				let mut image = RgbImage::new(wd, hg);
				draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(wd, hg), grn);
				draw_text_mut(&mut image, wht, mx, my, scale, &font, &o);
				draw_text_mut(&mut image, wht, 6, 2, small, &font, &format!("or-{oid}"));
				let adw = if maxwd>wd { (wd-mx as u32).try_into().unwrap() } else { (maxwd-mx as u32).try_into().unwrap() };
				draw_filled_rect_mut(&mut image, Rect::at(adw, 0).of_size(mx as u32, hg), grn);
				if wd>=maxwd { draw_text_mut(&mut image, wht, adw, 0, scale, &font, "..."); }
				for i in 0..2 {
					let ff : f32 = i as f32;
					draw_line_segment_mut(&mut image, (0.0,0.0+ff), (wdf,0.0+ff), wht);
					draw_line_segment_mut(&mut image, (0.0,hgf-1.0-ff), (wdf-1.0,hgf-1.0-ff), wht);
					draw_line_segment_mut(&mut image, (0.0+ff,0.0), (0.0+ff,hgf-1.0), wht);
					draw_line_segment_mut(&mut image, (wdf-1.0-ff,0.0), (wdf-1.0-ff,hgf-1.0), wht);
				}
				let path = format!("{pic_fld}/or-{oid}.png");
				let subimg = imageops::crop(&mut image, 0, 0, maxwd, hg);
				let _ = subimg.to_image().save(&path).unwrap();
				//==== org name END

				//==== service name BEGIN
				let (w, h) = text_size(scale, &font, &s.srv);
				let wd :u32 = (w + mx * 2).try_into().unwrap();
				let hg :u32 = (h + my * 2).try_into().unwrap();
				let wdf = wd as f32;
				let hgf = hg as f32;
				let mut image = RgbImage::new(wd, hg);
				draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(wd, hg), grn);
				draw_text_mut(&mut image, wht, mx, my, scale, &font, &s.srv);
				draw_text_mut(&mut image, wht, 6, 2, small, &font, &format!("sv-{oid}"));
				let adw = if maxwd>wd { (wd-mx as u32).try_into().unwrap() } else { (maxwd-mx as u32).try_into().unwrap() };
				draw_filled_rect_mut(&mut image, Rect::at(adw, 0).of_size(mx as u32, hg), grn);
				if wd>=maxwd { draw_text_mut(&mut image, wht, adw, 0, scale, &font, "..."); }

				for i in 0..2 {
					let ff : f32 = i as f32;
					draw_line_segment_mut(&mut image, (0.0,0.0+ff), (wdf,0.0+ff), wht);
					draw_line_segment_mut(&mut image, (0.0,hgf-1.0-ff), (wdf-1.0,hgf-1.0-ff), wht);
					draw_line_segment_mut(&mut image, (0.0+ff,0.0), (0.0+ff,hgf-1.0), wht);
					draw_line_segment_mut(&mut image, (wdf-1.0-ff,0.0), (wdf-1.0-ff,hgf-1.0), wht);
				}
				let path = format!("{pic_fld}/sv-{sid}.png");
				let subimg = imageops::crop(&mut image, 0, 0, maxwd, hg);
				let _ = subimg.to_image().save(&path).unwrap();
				//==== service name END

				println!("{}-{}-{}", s.srv, s.org, s.no);
				break;
			} // for s
		} // Some(srv)
	}
	Ok(())
}

