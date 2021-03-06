// Copyright 2019 CoreOS, Inc.
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

use error_chain::error_chain;

error_chain! {
    foreign_links {
        GptMan(gptman::Error);
        HexDecode(hex::FromHexError);
        Io(std::io::Error);
        Reqwest(reqwest::Error);
        Url(url::ParseError);
        Nix(nix::Error);
        WalkDir(walkdir::Error);
        Parse(std::num::ParseIntError);
        Xz(xz2::stream::Error);
    }
}
