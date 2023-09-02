use std::env;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;
use ureq::Agent;

fn main() -> anyhow::Result<()> {
    let agent: Agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();

    download_and_strip_eff_file_to_dir(
        &agent,
        "https://www.eff.org/files/2016/09/08/eff_short_wordlist_2_0.txt",
        "eff_short_wordlist_2_0.txt",
    )?;

    download_and_strip_eff_file_to_dir(
        &agent,
        "https://www.eff.org/files/2016/09/08/eff_short_wordlist_1.txt",
        "eff_short_wordlist_1.txt",
    )?;

    download_and_strip_eff_file_to_dir(
        &agent,
        "https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt",
        "eff_large_wordlist.txt",
    )?;

    // all github downloads are locked to a commit for reproducibility
    // this may not be the right choice. But works for now

    // https://github.com/andreasonny83/unique-names-generator
    // commit 10ff70b131c8a080e88c315a55e45a0f5caadd24
    // head branch main
    download_unique_names_generator_file(&agent, "adjectives")?;
    download_unique_names_generator_file(&agent, "animals")?;
    download_unique_names_generator_file(&agent, "colors")?;
    download_unique_names_generator_file(&agent, "countries")?;
    download_unique_names_generator_file(&agent, "names")?;
    download_unique_names_generator_file(&agent, "star-wars")?;

    // https://github.com/a-type/adjective-adjective-animal
    // commit 877ec25a2d0b54044f8a60b50cc2456a80345910
    // head branch master
    download_code_list_file_to_text_file(
        &agent,
        "https://raw.githubusercontent.com/a-type/adjective-adjective-animal/877ec25a2d0b54044f8a60b50cc2456a80345910/lib/lists/adjectives.js",
        "adjective_adjective_animal_adjectives.txt",
    )?;

    download_code_list_file_to_text_file(
        &agent,
        "https://raw.githubusercontent.com/a-type/adjective-adjective-animal/877ec25a2d0b54044f8a60b50cc2456a80345910/lib/lists/animals.js",
        "adjective_adjective_animal_animals.txt",
    )?;

    // https://github.com/Exr0nProjects/witty-phrase-generator
    // commit 67b8d82e4e80eeae928c82c917d8d04ff6464343
    // head branch main
    download_file_to_dir(
        &agent,
        "https://raw.githubusercontent.com/Exr0nProjects/witty-phrase-generator/67b8d82e4e80eeae928c82c917d8d04ff6464343/src/adjectives.txt",
        "witty_phrase_generator_adjectives.txt",
    )?;

    download_file_to_dir(
        &agent,
        "https://raw.githubusercontent.com/Exr0nProjects/witty-phrase-generator/67b8d82e4e80eeae928c82c917d8d04ff6464343/src/intensifiers.txt",
        "witty_phrase_generator_intensifiers.txt",
    )?;

    download_file_to_dir(
        &agent,
        "https://raw.githubusercontent.com/Exr0nProjects/witty-phrase-generator/67b8d82e4e80eeae928c82c917d8d04ff6464343/src/nouns.txt",
        "witty_phrase_generator_nouns.txt",
    )?;

    // https://github.com/droundy/memorable-wordlist
    // commit fdeaa5452e884249c5d63e3d64d94f3e948014a4
    // head branch master
    download_code_list_file_to_text_file(
        &agent,
        "https://raw.githubusercontent.com/droundy/memorable-wordlist/fdeaa5452e884249c5d63e3d64d94f3e948014a4/src/words.rs",
        "memorable_wordlist_words.txt",
    )?;

    Ok(())
}

fn download_file_to_dir(agent: &Agent, url: &str, file_name: &str) -> anyhow::Result<()> {
    let content: String = agent.get(url).call()?.into_string()?;
    let out_dir = PathBuf::from_str(&env::var("OUT_DIR")?)?;

    std::fs::write(out_dir.join(file_name), content)?;
    Ok(())
}

fn download_and_strip_eff_file_to_dir(
    agent: &Agent,
    url: &str,
    file_name: &str,
) -> anyhow::Result<()> {
    let content: String = agent.get(url).call()?.into_string()?;

    // eff files start with N digits and a tab so we strip it
    let content_stripped = content
        .lines()
        .filter_map(|line| {
            let number_of_digits = line.chars().take_while(|c| c.is_ascii_digit()).count();
            line.get(number_of_digits..)
        })
        .map(|line| line.trim().to_owned())
        .collect::<Vec<_>>()
        .join("\n");

    let out_dir = PathBuf::from_str(&env::var("OUT_DIR")?)?;
    std::fs::write(out_dir.join(file_name), content_stripped)?;
    Ok(())
}

/// supports some typescript, javascript, and rust files
fn download_code_list_file_to_text_file(
    agent: &Agent,
    url: &str,
    file_name: &str,
) -> anyhow::Result<()> {
    let content: String = agent.get(url).call()?.into_string()?;

    // strip typescript list elements from content
    let content_stripped = content
        .lines()
        .filter(|line| !line.starts_with("export default ["))
        .filter(|line| !line.starts_with("module.exports = ["))
        .filter(|line| !line.starts_with("pub const LIST: &[&str] = &["))
        .filter(|line| !line.starts_with("];"))
        .map(|line| line.replace(['\'', ',', '\"'], "").trim().to_owned())
        .collect::<Vec<_>>()
        .join("\n");

    let out_dir = PathBuf::from_str(&env::var("OUT_DIR")?)?;

    std::fs::write(out_dir.join(file_name), content_stripped)?;
    Ok(())
}

fn download_unique_names_generator_file(agent: &Agent, file_name: &str) -> anyhow::Result<()> {
    download_code_list_file_to_text_file(
        agent,
        &format!("https://raw.githubusercontent.com/andreasonny83/unique-names-generator/10ff70b131c8a080e88c315a55e45a0f5caadd24/src/dictionaries/{}.ts", file_name),
        &format!("unique_names_generator_{}.txt", file_name),
    )?;
    Ok(())
}
