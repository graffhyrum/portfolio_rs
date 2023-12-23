use askama::{Template};
use axum::response::IntoResponse;

pub async fn my_work() -> impl IntoResponse {
    MyWorkTemplate {
        sections: vec![
            Section {
                title: "Projects",
                subtitle: "This is a collection of projects that I have worked on.",
                content: vec![
                    ContentEntry {
                        title: "This Website",
                        href: "/",
                        content: "This website is built using Rust, Axum, and Askama.",
                    },
                    ContentEntry {
                        title: "Playwright JSON Summary Reporter",
                        href: "https://github.com/graffhyrum/playwright-json-summary-reporter",
                        content: "A custom reporter for Playwright that outputs a JSON summary of test results.
        I contributed to this project by adding a new feature that allows the user to specify a custom output file
        path.",
                    },
                    ContentEntry {
                        title: "Playwright Project Builder",
                        href: "https://github.com/graffhyrum/playwright-project-builder",
                        content: "A Playwright test project config factory. It is designed to
be a starting point for Playwright projects. By setting environment variables for
each environment you would want to test, you can execute a setup project,
your test suite against all enabled environments, and a teardown project. ",
                    },
                    ContentEntry {
                        title: "OvationCXM",
                        href: "https://www.ovationcxm.com/",
                        content: "A customer experience management platform that helps businesses collect and analyze customer feedback.
        I contributed to this project by triaging (and sometimes fixing) bugs, and was the primary developer for the automated testing framework.",
                    },
                ],
            },
            Section {
                title: "Gists",
                subtitle: "This is a collection of gists that I have created.",
                content: vec![
                    ContentEntry {
                        title: "Typescript - Recursive Partial Type",
                        href: "https://gist.github.com/graffhyrum/7f267cea2021ad4246be23ec5f6d4a4e",
                        content: r#"
                                    <p>
        A recursive version of the Partial type. The RecursivePartial utility type can be used in scenarios where you
        want to make all properties of an object (including nested properties) optional.
      </p>
      <pre>
      <code class="language-typescript">
/**
 * A custom utility type to allow an object to be only one Type from the provided
 * Type.
 * @example
 * type OneOf = OneOfType&lt;{userId:string, userEmail:string}&gt;;
 * //allows {userId:string} || {userEmail:string}, but not both
 */
export type OneOfType&lt;T&gt; = ValueOf&lt;OneOfByKey&lt;T&gt;&gt;;
type OneOfByKey&lt;T&gt; = {[key in keyof T]: OneOnly&lt;T, key&gt;};
type OneOnly&lt;Obj, Key extends keyof Obj&gt; = {
  [key in Exclude&lt;keyof Obj, Key&gt;]+?: undefined;
} &amp; Pick&lt;Obj, Key&gt;;
type ValueOf&lt;Obj extends object&gt; = Obj[keyof Obj];
      </code>
      </pre>
                        "#,
                    },
                    ContentEntry {
                        title: "Typescript - 'One of' Type",
                        href: "https://gist.github.com/graffhyrum/d705dc05cf3890303dd9aa1c9598b08d",
                        content: r#"
                        <p>
        A utility type OneOfType that ensures an object can only have one property from the provided type. This is
        achieved using TypeScript's conditional and mapped types.
      </p>
      <pre>
      <code class="language-typescript">
/**
 * A custom utility type to allow an object to be only one Type from the provided
 * Type.
 * @example
 * type OneOf = OneOfType&lt;{userId:string, userEmail:string}&gt;;
 * //allows {userId:string} || {userEmail:string}, but not both
 */
export type OneOfType&lt;T&gt; = ValueOf&lt;OneOfByKey&lt;T&gt;&gt;;
type OneOfByKey&lt;T&gt; = {[key in keyof T]: OneOnly&lt;T, key&gt;};
type OneOnly&lt;Obj, Key extends keyof Obj&gt; = {
  [key in Exclude&lt;keyof Obj, Key&gt;]+?: undefined;
} &amp; Pick&lt;Obj, Key&gt;;
type ValueOf&lt;Obj extends object&gt; = Obj[keyof Obj];
      </code>
      </pre>
      "#,
                    },
                    ContentEntry {
                        title: "Typescript - Type-safe Entries",
                        href: "https://gist.github.com/graffhyrum/1253b24fbe80d5f508544736d83d9532",
                        content: r#"
<p>
    A utility for working with objects in a type-safe manner. It defines a type Entries and a function getEntries.
</p>
<pre>
<code class="language-typescript">
export type Entries&ltT&gt = {
  [K in keyof T]-?: [K, T[K]];
}[keyof T][];

export const getEntries = &ltT extends object&gt(obj: T): Entries&ltT&gt =&gt {
  return Object.entries(obj) as Entries&ltT&gt;
};

const exampleObj = {
  name: 'John',
  age: 30,
  location: 'New York',
};

const entries = getEntries(exampleObj);
// type of entries is:
// type Entries = ["name", string] | ["age", number] | ["location", string]
const entries2 = Object.entries(exampleObj);
// type of entries2 is:
// type Entries2 = [keyof typeof exampleObj, string | number][]
</code>
</pre>
"#,
                    },
                    ContentEntry {
                        title: "Typescript - Branded Types",
                        href: "https://gist.github.com/graffhyrum/bdf39a9e7fe18876fcc1dabf11c92457",
                        content: r#"
                      <p>
        A "Branded" type is a type that is a subtype of the original type, but has a unique literal value in a common
        field (the brand). This allows us to define types that are more specific than the original type, but are still
        compatible with it.
      </p>
      <pre>
      <code class="language-typescript">
export type EmailAddress = string & {__brand: 'emailAddress'};

/**
 * EmailAddress assertion function
 */
export function assertIsEmailAddress(
  emailAddress: unknown
): asserts emailAddress is EmailAddress {
  if (!isEmailAddress(emailAddress)) {
    throw new Error(`Expected ${emailAddress} to be a valid email address`);
  }
}

/**
 * Email address type check function
 */
export function isEmailAddress(
  emailAddress: unknown
): emailAddress is EmailAddress {
  const emailRegex = /^[\w-.]+@([\w-]+\.)+[\w-]{2,4}$/;
  if (typeof emailAddress === 'string') {
    return emailAddress.match(emailRegex) !== null;
  } else {
    return false;
  }
}
      </code>
      </pre>
      "#,
                    },
                ],
            },
        ],
    }
}

#[derive(Template)]
#[template(path = "pages/my_work.html")]
struct MyWorkTemplate {
    sections: Vec<Section>,
}

struct Section {
    title: &'static str,
    subtitle: &'static str,
    content: Vec<ContentEntry>,
}

struct ContentEntry {
    title: &'static str,
    href: &'static str,
    // this is marked as 'safe' in the template. be sure to use a raw string literal (r#""#)
    // if you want to include HTML tags in the content.
    content: &'static str,
}

