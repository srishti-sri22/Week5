mod split;
mod verify_secret;
mod verify_share;
mod reconstruct;

use crate::cli::Command;

pub fn execute(cmd: Command) {
    match cmd {
        Command::Split { secret, n, k } => {
            split::execute(secret, n, k);
        }
        
        Command::VerifySecret { secret, commitments } => {
            verify_secret::execute(secret, commitments);
        }
        
        Command::VerifyShare { share, commitments, verbose } => {
            verify_share::execute(share, commitments, verbose);
        }
        
        Command::Reconstruct { shares } => {
            reconstruct::execute(shares);
        }
    }
}