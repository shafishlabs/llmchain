// Copyright 2023 Shafish Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::io::Write;

use anyhow::Result;
use goldenfile::Mint;
use llmchain_loaders::DocumentLoader;
use llmchain_loaders::DocumentPath;
use llmchain_loaders::LocalDisk;
use llmchain_loaders::PdfLoader;

#[tokio::test]
async fn test_pdf_loader() -> Result<()> {
    // testdata dir.
    let curdir = std::env::current_dir()?.to_str().unwrap().to_string();
    let testdata_dir = format!("{}/tests/testdata", curdir);
    let pdf_file = format!("{}/pdf/2203.02155.pdf", testdata_dir);

    // Load
    let pdf_loader = PdfLoader::create(LocalDisk::create()?);
    let documents = pdf_loader
        .load(DocumentPath::from_string(&pdf_file))
        .await?;

    // Check.
    let mut mint = Mint::new(&testdata_dir);
    let golden_path = "markdown/copy_md_loader.golden";
    let mut file = mint.new_goldenfile(golden_path)?;
    for (i, doc) in documents.iter().enumerate() {
        writeln!(
            file,
            "part={}, len={}, md5={}",
            i,
            doc.content.len(),
            doc.content_md5
        )?;
        writeln!(
            file,
            "------------------------------------------------------------"
        )?;
        writeln!(file, "{}", doc.content)?;
    }

    Ok(())
}
