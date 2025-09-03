// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Cody Carlock

//! Author: Cody Carlock (codycarlock.77@gmail.com)
//! Since: 2025-08-27
//!
//! All content within this repository follows the above license unless explicitly stated otherwise.
//! Documentation, online forums, etc. were used in this project. Various large-language models (LLMs) were used for research and guidance.
//! No code was directly copied from any source, including LLMs. LLMs helped with code analysis for streamlined documentation.

/*
Developer note:
- This program serves as a master collection of practice RUST projects intended to show my affinity to learn and manipulate code.
- Prior to starting this project, I had never used RUST (or any other lower-level language).
- The project demonstrates my ability to adapt to and overcome new challenges, even when entirely unfamiliar to me.
*/

mod util;
mod projects;

fn main() {
    println!("Running main!");
    projects::demo1_temp_converter::run();
}
