module main

import os
import v.vmod

const v_mod_path = os.resource_abs_path("../v.mod")
const manifest = vmod.from_file(v_mod_path) or { panic(err) }

pub const (
	version     = manifest.version
	name        = manifest.name
	description = manifest.description
) 

fn main() {
	println('CLI v${main.version} runs!')
}
