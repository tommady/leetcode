// Every email consists of a local name and a domain name, separated by the @ sign.
//
// For example, in alice@leetcode.com, alice is the local name, and leetcode.com is the domain name.
//
// Besides lowercase letters, these emails may contain &#39;.&#39;s or &#39;+&#39;s.
//
// If you add periods (&#39;.&#39;) between some characters in the local name part of an email address, mail sent there will be forwarded to the same address without dots in the local name. For example, alice.z@leetcode.com and alicez@leetcode.com forward to the same email address. (Note that this rule does not apply for domain names.)
//
// If you add a plus (&#39;+&#39;) in the local name, everything after the first plus sign will be ignored. This allows certain emails to be filtered, for example m.y+name@email.com will be forwarded to my@email.com. (Again, this rule does not apply for domain names.)
//
// It is possible to use both of these rules at the same time.
//
// Given a list of emails, we send one email to each address in the list. How many different addresses actually receive mails?
//
//
//
//
// Example 1:
//
//
// Input: [ test.email+alex@leetcode.com , test.e.mail+bob.cathy@leetcode.com , testemail+david@lee.tcode.com ]
// Output: 2
// Explanation: testemail@leetcode.com and testemail@lee.tcode.com actually receive mails
//
//
//
//
// Note:
//
//
// 	1 = emails[i].length = 100
// 	1 = emails.length = 100
// 	Each emails[i] contains exactly one &#39;@&#39; character.
// 	All local and domain names are non-empty.
// 	Local names do not start with a &#39;+&#39; character.
//
//
//

#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        use std::collections::HashSet;
        let mut unique_emails = HashSet::new();

        for email in emails.iter() {
            let email: Vec<&str> = email.split('@').collect();
            let mut local = email[0].to_owned();

            if let Some(plus) = local.find('+') {
                local.replace_range(plus..local.len(), "")
            }
            local = local.replace('.', "");

            unique_emails.insert(local + "@" + email[1]);
        }

        unique_emails.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_929_solution() {
        assert_eq!(
            2,
            Solution::num_unique_emails(vec![
                "test.email+alex@leetcode.com".to_owned(),
                "test.e.mail+bob.cathy@leetcode.com".to_owned(),
                "testemail+david@lee.tcode.com".to_owned()
            ])
        )
    }
}
