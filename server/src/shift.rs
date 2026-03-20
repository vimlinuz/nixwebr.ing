use crate::types::{WebringMember, WebsiteStatus};

pub fn shift_ring(
    members: &[WebringMember],
    name: &str,
    forward: bool,
) -> Option<String> {
    if let Some((i, _)) = members.iter().enumerate().find(|(_, member)| member.name == *name) {
        let mut m = members.to_owned().clone();
        if forward {
            m.rotate_left(i + 1);
        } else {
            m.rotate_left(i);
            m.reverse();
        }

        //WARN: Don't know what is the reason for having the sites with broken links in the list
        let next_index = m.iter()
            .enumerate()
            .find(|(_, WebringMember { ref site_status, .. })|
                *site_status == WebsiteStatus::Ok
                || *site_status == WebsiteStatus::BrokenLinks
            )
            .map(|(i, _)| i);

        let next_site = match next_index {
            Some(i) => &m[i].site,
            None => "https://nixwebr.ing/",
        };

        return Some(next_site.to_string());
    }

    None
}
