use super::data::Chain;

use os_info::Type as OSType;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tracing::*;
// A function that finds all the installed chains: VRSC, VRSCTEST and the used PBaaS installations
// (.komodo/VRSC, .komodo/VRSCTEST, .verustest/pbaas/* )
pub(crate) fn find_local_chain_installations() -> Arc<Vec<Arc<Chain>>> {
    let mut installations = vec![];

    if let Some(homedir) = dirs::home_dir() {
        // First find Verus and Verustest:
        let komodo_path = match os_info::get().os_type() {
            OSType::Ubuntu | OSType::Linux => {
                PathBuf::from(&format!("{}/.komodo", &homedir.to_str().unwrap()))
            }
            OSType::Macos | OSType::Windows => {
                PathBuf::from(&format!("{}/Komodo", &homedir.to_str().unwrap()))
            }
            _ => panic!("unsupported OS"),
        };

        if Path::new(&format!(
            "{}/VRSC/VRSC.conf",
            komodo_path.to_str().expect("a valid path")
        ))
        .exists()
        {
            debug!("VRSC config has been found");
            installations.push(Arc::new(Chain::new("VRSC")));
        }

        if Path::new(&format!(
            "{}/vrsctest/vrsctest.conf",
            komodo_path.to_str().expect("a valid path")
        ))
        .exists()
        {
            debug!("verustest config has been found");
            installations.push(Arc::new(Chain::new("vrsctest")));
        }

        let verustest_path = match os_info::get().os_type() {
            OSType::Ubuntu | OSType::Linux => {
                PathBuf::from(&format!("{}/.verustest/pbaas", &homedir.to_str().unwrap()))
            }
            OSType::Macos | OSType::Windows => {
                PathBuf::from(&format!("{}/VerusTest/pbaas", homedir.to_str().unwrap()))
            }
            _ => panic!("unsupported OS"),
        };

        // traverse the pbaas directory and find installed pbaas chains.
        // TODO: there could be old VRSCTEST installations! users should always empty .verustest so panic should be fine.
        for entry in verustest_path.read_dir().expect("a dir") {
            if let Ok(entry) = entry {
                if let Ok(pbaasentry) = PathBuf::from(&entry.path()).read_dir() {
                    for i in pbaasentry.into_iter() {
                        if let Ok(direntry) = i {
                            match direntry.file_name().to_str() {
                                Some(filename) => {
                                    if filename
                                        == format!(
                                            "{}.conf",
                                            &entry.file_name().into_string().unwrap()
                                        )
                                        .as_str()
                                    {
                                        let pathbuf = PathBuf::from(&direntry.path());

                                        installations.push(Arc::new(Chain::new(format!(
                                            "{}",
                                            pathbuf
                                                .file_prefix()
                                                .unwrap()
                                                .to_owned()
                                                .into_string()
                                                .unwrap()
                                        ))));
                                    }
                                }
                                None => {} //invalid (non-ascii) filenames will never be chains in verus
                            }
                        }
                    }
                }
            }
        }
    } else {
        panic!("unsupported OS (no home directory found)");
    };

    Arc::new(installations)
}
