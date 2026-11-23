use crate::message::Message;
use colored::{ColoredString, Colorize};
use rust_apt::{error::AptErrors, util as apt_util};

/// Handle APT errors.
pub fn handle_errors(errors: AptErrors) {
    let mut makedeb_msg = Message::new();

    for err in errors.iter() {
        if err.is_error {
            makedeb_msg = makedeb_msg.error(&err.msg);
        } else {
            makedeb_msg = makedeb_msg.warning(&err.msg);
        }
    }

    makedeb_msg.send();
}

pub fn format_apt_pkglist<T: AsRef<str>>(pkgnames: &[T]) -> ColoredString {
    // All package lines always start with two spaces, so pretend like we have two
    // less characters.
    let term_width = apt_util::terminal_width() - 2;

    let mut output = String::from("  ");
    let mut current_width = 0;

    for (index, pkgname_ref) in pkgnames.iter().enumerate() {
        let pkgname = pkgname_ref.as_ref();
        output.push_str(pkgname);
        // Add 1 for the space that'll appear if we add another package on this line.
        current_width += pkgname.len() + 1;

        let next_pkgname = match pkgnames.get(index + 1) {
            Some(string) => string.as_ref(),
            None => "",
        };

        if current_width + next_pkgname.len() > term_width {
            output.push_str("\n  ");
            current_width = 0;
        } else {
            output.push(' ');
        }
    }

    output.bold()
}

/// Lock the cache.
pub fn lock_cache() {
    if let Err(err) = apt_util::apt_lock() {
        handle_errors(err);
        Message::new().error("Couldn't lock the APT cache.").send();
    } else if let Err(err) = apt_util::apt_lock_inner() {
        handle_errors(err);
        Message::new().error("Couldn't lock the APT cache.").send();
    }
}

/// Unlock the cache.
pub fn unlock_cache() {
    apt_util::apt_unlock();
    apt_util::apt_unlock_inner();
}
