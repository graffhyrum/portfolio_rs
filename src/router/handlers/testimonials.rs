use askama::Template;
use axum::response::IntoResponse;

pub async fn testimonial() -> impl IntoResponse {
    TestimonialTemplate {
        entries: vec![
            ContentEntry {
                title: "Mike Schwartz | COO @ OvationCXM",
                href: "https://www.linkedin.com/in/michaeljasonschwartz",
                description: "I am delighted to recommend Josh as a dedicated professional with a brilliant mind and an unwavering commitment to quality.
Josh consistently demonstrates an impressive level of intelligence and a strong attention to detail in all aspects of his work. I have had the pleasure of witnessing his continuous desire to refine processes, always seeking innovative solutions to enhance both efficiency and effectiveness.
Josh is a highly skilled SDET and an insightful contributor to the continuous improvement of our testing practices. Josh's intellectual curiosity and commitment to excellence make him a great asset to any team fortunate enough to have him.",
            },
            ContentEntry {
                title: "John States | Strategic Product Manager @ OvationCXM",
                href: "https://www.linkedin.com/in/johnstates001/",
                description: "Josh is an amazing SDET and coworker. His attention to detail and technical acumen are second to none. Was instrumental in developing our entire automated test suite.",
            },
            ContentEntry {
                title: "Daniel Konen | Product Manager @ TRAY",
                href: "https://www.linkedin.com/in/danielkonen/",
                description: "Josh is a great team member to work with. He has great organizational skills and is always looking to improve a process that can be better automated.
Josh also knows how to provide a great customer experience. Some customers can be problematic, but Josh knows how to properly communicate with them and excels at providing great expectations.
",
            },
            ContentEntry {
                title: "Chris Hall | Director of Software Architecture @ OvationCXM",
                href: "https://www.linkedin.com/in/chris-hall-7b94a7265/",
                description: "Josh is a great team member to work with. He has great organizational skills and is always looking to improve a process that can be better automated."
            },
            ContentEntry {
                title: "TRAY Hero of the Quarter - Q2 2019",
                href: "https://www.linkedin.com/company/vendsy-inc-/",
                description: "The TRAY Hero award is the most prestigious honor the company awards to the employee who during the previous quarter has exemplified TRAY's values the most:
Integrity and team spirit
Going above and beyond
Work ethic and significant contributions

Josh has performed his duties with exemplary work ethic and teamwork. He also went above and beyond and made significant contributions to TRAY:

He helped resolve Rush Fun Park’s numerous issues by spending multiple days on-site monitoring, replacing and fixing hardware as well as providing network configuration assistance. This resulted in a successful customer testimonial video.

Josh introduced and helped implement multiple key tools and processes we use every day including Trello which we use for on-boarding.

Josh is actively involved in our documentation efforts and is the first person to update existing documents when something changes or create content like in-depth video guides for large product features.

Josh submitted more product enhancements to JIRA than anyone else including great detail and UI examples."
            },
        ],
    }
}

#[derive(Template)]
#[template(path = "pages/testimonials.html")]
struct TestimonialTemplate<'a> {
    entries: Vec<ContentEntry<'a>>,
}

struct ContentEntry<'a> {
    title: &'a str,
    href: &'a str,
    description: &'a str,
}
