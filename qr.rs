//! Minimalny generator kodów QR (wersje 1-L..=6-L, tryb Byte, korekcja błędów L)
//! na licencji MIT, bez zewnętrznych zależności.

#[derive(Debug, Clone)]
pub struct QrCode {
    pub size: usize,
    pub modules: Vec<bool>,
}

impl QrCode {
    pub fn get(&self, row: usize, col: usize) -> bool {
        if row >= self.size || col >= self.size {
            return false;
        }
        self.modules[row * self.size + col]
    }

    pub fn to_svg(
        &self,
        module_px: u32,
        quiet_zone: u32,
        dark_color: &str,
        light_color: &str,
    ) -> String {
        let px = module_px.max(1);
        let qz = quiet_zone;
        let total_modules = self.size as u32 + qz * 2;
        let total_px = total_modules * px;

        let mut svg = String::new();
        svg.push_str(&format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {0} {0}" width="{0}" height="{0}" shape-rendering="crispEdges">"#,
            total_px
        ));
        svg.push_str(&format!(
            r#"<rect width="100%" height="100%" fill="{}"/>"#,
            escape_xml_attr(light_color)
        ));

        for r in 0..self.size {
            for c in 0..self.size {
                if self.get(r, c) {
                    let x = (c as u32 + qz) * px;
                    let y = (r as u32 + qz) * px;
                    svg.push_str(&format!(
                        r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}"/>"#,
                        x,
                        y,
                        px,
                        px,
                        escape_xml_attr(dark_color)
                    ));
                }
            }
        }

        svg.push_str("</svg>");
        svg
    }
}

fn escape_xml_attr(val: &str) -> String {
    val.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

struct VersionSpec {
    version: u8,
    size: usize,
    max_data_bytes: usize,
    data_codewords: usize,
    ecc_per_block: usize,
    blocks: usize,
    align_coords: &'static [usize],
}

const SPECS: [VersionSpec; 6] = [
    VersionSpec {
        version: 1,
        size: 21,
        max_data_bytes: 17,
        data_codewords: 19,
        ecc_per_block: 7,
        blocks: 1,
        align_coords: &[],
    },
    VersionSpec {
        version: 2,
        size: 25,
        max_data_bytes: 32,
        data_codewords: 34,
        ecc_per_block: 10,
        blocks: 1,
        align_coords: &[6, 18],
    },
    VersionSpec {
        version: 3,
        size: 29,
        max_data_bytes: 53,
        data_codewords: 55,
        ecc_per_block: 15,
        blocks: 1,
        align_coords: &[6, 22],
    },
    VersionSpec {
        version: 4,
        size: 33,
        max_data_bytes: 78,
        data_codewords: 80,
        ecc_per_block: 20,
        blocks: 1,
        align_coords: &[6, 26],
    },
    VersionSpec {
        version: 5,
        size: 37,
        max_data_bytes: 106,
        data_codewords: 108,
        ecc_per_block: 26,
        blocks: 1,
        align_coords: &[6, 30],
    },
    VersionSpec {
        version: 6,
        size: 41,
        max_data_bytes: 134,
        data_codewords: 136,
        ecc_per_block: 18,
        blocks: 2,
        align_coords: &[6, 34],
    },
];

pub fn encode_text(text: &str) -> Result<QrCode, String> {
    let bytes = text.as_bytes();
    if bytes.is_empty() {
        return Err("Tekst do zakodowania w QR nie może być pusty.".to_string());
    }

    let spec = SPECS
        .iter()
        .find(|s| bytes.len() <= s.max_data_bytes)
        .ok_or_else(|| {
            format!(
                "Tekst ma {} bajtów (obsługiwane wersje 1-L..6-L obsługują maks. 134 bajty).",
                bytes.len()
            )
        })?;

    let data_cw = build_data_codewords(bytes, spec.data_codewords);
    let interleaved_cw = interleave_codewords(&data_cw, spec.blocks, spec.ecc_per_block);

    let sz = spec.size;
    let mut matrix: Vec<Option<bool>> = vec![None; sz * sz];
    let mut reserved = vec![false; sz * sz];

    place_finder_patterns(&mut matrix, &mut reserved, sz);
    place_separators(&mut matrix, &mut reserved, sz);
    place_alignment_patterns(&mut matrix, &mut reserved, sz, spec.align_coords);
    place_timing_patterns(&mut matrix, &mut reserved, sz);
    place_dark_module(&mut matrix, &mut reserved, sz, spec.version);
    reserve_format_areas(&mut reserved, sz);

    place_data_bits(&mut matrix, &reserved, sz, &interleaved_cw);

    let mut best_mask = 0u8;
    let mut best_penalty = u32::MAX;
    for mask in 0..8u8 {
        let mut candidate = matrix.clone();
        apply_mask(&mut candidate, &reserved, sz, mask);
        write_format_info(&mut candidate, sz, mask);
        let penalty = evaluate_penalty(&candidate, sz);
        if penalty < best_penalty {
            best_penalty = penalty;
            best_mask = mask;
        }
    }

    apply_mask(&mut matrix, &reserved, sz, best_mask);
    write_format_info(&mut matrix, sz, best_mask);

    let modules = matrix.into_iter().map(|m| m.unwrap_or(false)).collect();
    Ok(QrCode {
        size: sz,
        modules,
    })
}

fn build_data_codewords(data: &[u8], total_data_cw: usize) -> Vec<u8> {
    let total_bits = total_data_cw * 8;
    let mut bits: Vec<u8> = Vec::with_capacity(total_bits);

    push_bits(&mut bits, 0b0100, 4);
    push_bits(&mut bits, data.len() as u32, 8);
    for &b in data {
        push_bits(&mut bits, b as u32, 8);
    }

    let term_len = (total_bits.saturating_sub(bits.len())).min(4);
    for _ in 0..term_len {
        bits.push(0);
    }

    while bits.len() % 8 != 0 {
        bits.push(0);
    }

    let mut codewords = vec![0u8; total_data_cw];
    let byte_count = (bits.len() / 8).min(total_data_cw);
    for i in 0..byte_count {
        let mut val = 0u8;
        for b in 0..8 {
            val = (val << 1) | bits[i * 8 + b];
        }
        codewords[i] = val;
    }

    let pad_bytes = [0xECu8, 0x11u8];
    let mut pad_idx = 0usize;
    for cw in codewords.iter_mut().skip(byte_count) {
        *cw = pad_bytes[pad_idx % 2];
        pad_idx += 1;
    }

    codewords
}

fn push_bits(out: &mut Vec<u8>, value: u32, count: u8) {
    for shift in (0..count).rev() {
        out.push(((value >> shift) & 1) as u8);
    }
}

fn interleave_codewords(data_cw: &[u8], blocks: usize, ecc_per_block: usize) -> Vec<u8> {
    let data_per_block = data_cw.len() / blocks;
    let mut block_data: Vec<&[u8]> = Vec::with_capacity(blocks);
    let mut block_ecc: Vec<Vec<u8>> = Vec::with_capacity(blocks);

    for b in 0..blocks {
        let start = b * data_per_block;
        let end = start + data_per_block;
        let slice = &data_cw[start..end];
        block_data.push(slice);
        block_ecc.push(rs_encode(slice, ecc_per_block));
    }

    let mut out = Vec::with_capacity(data_cw.len() + blocks * ecc_per_block);
    for i in 0..data_per_block {
        for b in 0..blocks {
            out.push(block_data[b][i]);
        }
    }
    for i in 0..ecc_per_block {
        for b in 0..blocks {
            out.push(block_ecc[b][i]);
        }
    }

    out
}

fn gf_mul(mut a: u8, mut b: u8) -> u8 {
    let mut p = 0u8;
    for _ in 0..8 {
        if (b & 1) != 0 {
            p ^= a;
        }
        let hi_bit = (a & 0x80) != 0;
        a <<= 1;
        if hi_bit {
            a ^= 0x1D;
        }
        b >>= 1;
    }
    p
}

fn gf_pow(exp: u8) -> u8 {
    let mut res = 1u8;
    for _ in 0..exp {
        res = gf_mul(res, 2);
    }
    res
}

fn build_rs_generator(ecc_len: usize) -> Vec<u8> {
    let mut poly = vec![1u8];
    for i in 0..ecc_len {
        let root = gf_pow(i as u8);
        let mut next = vec![0u8; poly.len() + 1];
        for (idx, &coeff) in poly.iter().enumerate() {
            next[idx] ^= coeff;
            next[idx + 1] ^= gf_mul(coeff, root);
        }
        poly = next;
    }
    poly[1..].to_vec()
}

fn rs_encode(data: &[u8], ecc_len: usize) -> Vec<u8> {
    let rs_gen = build_rs_generator(ecc_len);
    let mut ecc = vec![0u8; ecc_len];

    for &msg_byte in data {
        let factor = msg_byte ^ ecc[0];
        for i in 0..(ecc_len - 1) {
            ecc[i] = ecc[i + 1] ^ gf_mul(rs_gen[i], factor);
        }
        ecc[ecc_len - 1] = gf_mul(rs_gen[ecc_len - 1], factor);
    }

    ecc
}

fn set_mod(
    matrix: &mut [Option<bool>],
    reserved: &mut [bool],
    sz: usize,
    r: usize,
    c: usize,
    dark: bool,
    reserve: bool,
) {
    let idx = r * sz + c;
    matrix[idx] = Some(dark);
    if reserve {
        reserved[idx] = true;
    }
}

fn place_finder_patterns(matrix: &mut [Option<bool>], reserved: &mut [bool], sz: usize) {
    let origins = [(0usize, 0usize), (0usize, sz - 7), (sz - 7, 0usize)];
    for (top, left) in origins {
        for r in 0..7 {
            for c in 0..7 {
                let on_border = r == 0 || r == 6 || c == 0 || c == 6;
                let in_center = (2..=4).contains(&r) && (2..=4).contains(&c);
                let dark = on_border || in_center;
                set_mod(matrix, reserved, sz, top + r, left + c, dark, true);
            }
        }
    }
}

fn place_separators(matrix: &mut [Option<bool>], reserved: &mut [bool], sz: usize) {
    for i in 0..8 {
        if i < sz {
            set_mod(matrix, reserved, sz, 7, i, false, true);
            set_mod(matrix, reserved, sz, i, 7, false, true);
        }
    }
    for i in 0..8 {
        let c = sz - 8 + i;
        if c < sz {
            set_mod(matrix, reserved, sz, 7, c, false, true);
        }
        if i < sz {
            set_mod(matrix, reserved, sz, i, sz - 8, false, true);
        }
    }
    for i in 0..8 {
        let r = sz - 8 + i;
        if r < sz {
            set_mod(matrix, reserved, sz, r, 7, false, true);
        }
        if i < sz {
            set_mod(matrix, reserved, sz, sz - 8, i, false, true);
        }
    }
}

fn place_alignment_patterns(
    matrix: &mut [Option<bool>],
    reserved: &mut [bool],
    sz: usize,
    coords: &[usize],
) {
    for &center_r in coords {
        for &center_c in coords {
            if matrix[center_r * sz + center_c].is_some() {
                continue;
            }
            for dr in -2isize..=2 {
                for dc in -2isize..=2 {
                    let r = (center_r as isize + dr) as usize;
                    let c = (center_c as isize + dc) as usize;
                    let ad = dr.abs().max(dc.abs());
                    let dark = ad == 2 || ad == 0;
                    set_mod(matrix, reserved, sz, r, c, dark, true);
                }
            }
        }
    }
}

fn place_timing_patterns(matrix: &mut [Option<bool>], reserved: &mut [bool], sz: usize) {
    for i in 8..(sz - 8) {
        let dark = i % 2 == 0;
        if !reserved[6 * sz + i] {
            set_mod(matrix, reserved, sz, 6, i, dark, true);
        }
        if !reserved[i * sz + 6] {
            set_mod(matrix, reserved, sz, i, 6, dark, true);
        }
    }
}

fn place_dark_module(
    matrix: &mut [Option<bool>],
    reserved: &mut [bool],
    sz: usize,
    version: u8,
) {
    let r = (4 * version as usize) + 9;
    set_mod(matrix, reserved, sz, r, 8, true, true);
}

fn reserve_format_areas(reserved: &mut [bool], sz: usize) {
    for i in 0..=8 {
        reserved[8 * sz + i] = true;
        reserved[i * sz + 8] = true;
    }
    for c in (sz - 8)..sz {
        reserved[8 * sz + c] = true;
    }
    for r in (sz - 7)..sz {
        reserved[r * sz + 8] = true;
    }
}

fn place_data_bits(
    matrix: &mut [Option<bool>],
    reserved: &[bool],
    sz: usize,
    codewords: &[u8],
) {
    let mut bit_stream: Vec<u8> = Vec::with_capacity(codewords.len() * 8);
    for &cw in codewords {
        for shift in (0..8).rev() {
            bit_stream.push(((cw >> shift) & 1) as u8);
        }
    }

    let mut bit_idx = 0usize;
    let mut col = (sz as isize) - 1;
    let mut upward = true;

    while col > 0 {
        if col == 6 {
            col -= 1;
            continue;
        }

        for step in 0..sz {
            let row = if upward { sz - 1 - step } else { step };

            for dc in [0isize, -1isize] {
                let c = (col + dc) as usize;
                let idx = row * sz + c;
                if !reserved[idx] {
                    let dark = if bit_idx < bit_stream.len() {
                        bit_stream[bit_idx] == 1
                    } else {
                        false
                    };
                    matrix[idx] = Some(dark);
                    bit_idx += 1;
                }
            }
        }

        upward = !upward;
        col -= 2;
    }
}

fn mask_condition(mask: u8, r: usize, c: usize) -> bool {
    match mask {
        0 => (r + c) % 2 == 0,
        1 => r % 2 == 0,
        2 => c % 3 == 0,
        3 => (r + c) % 3 == 0,
        4 => (r / 2 + c / 3) % 2 == 0,
        5 => ((r * c) % 2) + ((r * c) % 3) == 0,
        6 => (((r * c) % 2) + ((r * c) % 3)) % 2 == 0,
        7 => (((r + c) % 2) + ((r * c) % 3)) % 2 == 0,
        _ => false,
    }
}

fn apply_mask(matrix: &mut [Option<bool>], reserved: &[bool], sz: usize, mask: u8) {
    for r in 0..sz {
        for c in 0..sz {
            let idx = r * sz + c;
            if !reserved[idx] && mask_condition(mask, r, c) {
                let cur = matrix[idx].unwrap_or(false);
                matrix[idx] = Some(!cur);
            }
        }
    }
}

fn format_bits_for_l(mask: u8) -> u16 {
    const FORMATS_L: [u16; 8] = [
        0x77C4, 0x72F3, 0x7DAA, 0x789D, 0x662F, 0x6318, 0x6C41, 0x6976,
    ];
    FORMATS_L[(mask & 7) as usize]
}

fn write_format_info(matrix: &mut [Option<bool>], sz: usize, mask: u8) {
    let bits = format_bits_for_l(mask);

    let primary_coords: [(usize, usize); 15] = [
        (0, 8),
        (1, 8),
        (2, 8),
        (3, 8),
        (4, 8),
        (5, 8),
        (7, 8),
        (8, 8),
        (8, 7),
        (8, 5),
        (8, 4),
        (8, 3),
        (8, 2),
        (8, 1),
        (8, 0),
    ];

    let secondary_coords: [(usize, usize); 15] = [
        (8, sz - 1),
        (8, sz - 2),
        (8, sz - 3),
        (8, sz - 4),
        (8, sz - 5),
        (8, sz - 6),
        (8, sz - 7),
        (8, sz - 8),
        (sz - 7, 8),
        (sz - 6, 8),
        (sz - 5, 8),
        (sz - 4, 8),
        (sz - 3, 8),
        (sz - 2, 8),
        (sz - 1, 8),
    ];

    for i in 0..15 {
        let shift = 14 - i;
        let dark = ((bits >> shift) & 1) != 0;
        let (pr, pc) = primary_coords[i];
        matrix[pr * sz + pc] = Some(dark);
        let (sr, sc) = secondary_coords[i];
        matrix[sr * sz + sc] = Some(dark);
    }
}

fn evaluate_penalty(matrix: &[Option<bool>], sz: usize) -> u32 {
    let is_dark = |r: usize, c: usize| matrix[r * sz + c].unwrap_or(false);
    let mut penalty = 0u32;

    for r in 0..sz {
        let mut run = 1u32;
        for c in 1..sz {
            if is_dark(r, c) == is_dark(r, c - 1) {
                run += 1;
            } else {
                if run >= 5 {
                    penalty += 3 + (run - 5);
                }
                run = 1;
            }
        }
        if run >= 5 {
            penalty += 3 + (run - 5);
        }
    }

    for c in 0..sz {
        let mut run = 1u32;
        for r in 1..sz {
            if is_dark(r, c) == is_dark(r - 1, c) {
                run += 1;
            } else {
                if run >= 5 {
                    penalty += 3 + (run - 5);
                }
                run = 1;
            }
        }
        if run >= 5 {
            penalty += 3 + (run - 5);
        }
    }

    for r in 0..(sz - 1) {
        for c in 0..(sz - 1) {
            let v = is_dark(r, c);
            if is_dark(r, c + 1) == v && is_dark(r + 1, c) == v && is_dark(r + 1, c + 1) == v {
                penalty += 3;
            }
        }
    }

    let mut dark_count = 0usize;
    for r in 0..sz {
        for c in 0..sz {
            if is_dark(r, c) {
                dark_count += 1;
            }
        }
    }
    let total = sz * sz;
    let pct = (dark_count * 100) / total;
    let prev_5 = (pct / 5) * 5;
    let next_5 = prev_5 + 5;
    let d1 = prev_5.abs_diff(50);
    let d2 = next_5.abs_diff(50);
    penalty += (d1.min(d2) as u32) * 2;

    penalty
}