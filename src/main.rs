use async_openai::{
    types::{
        ChatCompletionRequestMessageContentPartImageArgs,
        ChatCompletionRequestMessageContentPartTextArgs, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs, ImageDetail,
        ImageUrlArgs,
    },
    Client,
};
use std::error::Error;
use termimad::crossterm::style::Color::*;
use termimad::*;

// Fix: Change the main function to be asynchronous
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Processing image...");

    let client = Client::new();
    // let image_url = "https://upload.wikimedia.org/wikipedia/commons/thumb/d/dd/Gfp-wisconsin-madison-the-nature-boardwalk.jpg/2560px-Gfp-wisconsin-madison-the-nature-boardwalk.jpg";

    // Load the image from the current directory
    // let image_data = std::fs::read("notes.png")?;

    // Convert the image data to base64
    let base64_image = image_base64::to_base64("notes.png");
    let image_url = base64_image;

    // Create the data URL for the image
    // let image_url = format!("data:image/png;base64,{}", base64_image);

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        // .model("gpt-4o-mini") //or without mini for complex stuff
        .model("gpt-4o") //or without mini for complex stuff
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a tool to transcribe handwritten notes (bilingual English and German) into Markdown.\
                    Notes will be in BuJo style. When transcribing, pay attention to indentations, but don't overdo it.\
                    Do not wrap your output in triple ticks. Prefer use of unicode characters over emojis where possible.
                    Make sure to format the output as Markdown without empty lines.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(vec![
                    ChatCompletionRequestMessageContentPartTextArgs::default()
                        .text("Transcribe the image into Markdown")
                        .build()?
                        .into(),
                    ChatCompletionRequestMessageContentPartImageArgs::default()
                        .image_url(
                            ImageUrlArgs::default()
                                .url(image_url)
                                .detail(ImageDetail::High)
                                .build()?,
                        )
                        .build()?
                        .into(),
                ])
                .build()?
                .into(),
        ])
        .build()?;

    let response = client.chat().create(request).await?;

    let msg = response.choices[0]
        .message
        .content
        .clone()
        .unwrap_or_default();
    // let response = "\n## 2024-08-29 Mi\n\n- Streit mit Finn beim Frühstück wg. Wrap \n  - → Süßigkeiten-Verbot\n  - = :(\n  \n- ~~Höhe schneiden, Bio Müll~~\n- Reifen bestellen → 30 min\n- ~~Staub wischen~~\n\n- Rücksendungen: Ugreen, EA500, KR440\n\n- Mittag grillen\n\n- 15:30 Kaffee, Kuchen\n\n- = Bauchweh :(\n  - Fühle mich schwachlich (Arme, Beine)\n";

    println!("Response: {:?}", msg);
    let mut skin = MadSkin::default();
    skin.set_headers_fg(rgb(255, 187, 0));
    skin.bold.set_fg(Yellow);
    skin.italic.set_fgbg(Magenta, rgb(30, 30, 40));
    skin.bullet = StyledChar::from_fg_char(Yellow, '⟡');
    skin.quote_mark.set_fg(Yellow);
    // skin.print_inline(&msg);
    termimad::print_inline(&msg);

    // println!("{}", skin.term_text(response.as_str()));

    println!("\n\nDone!");

    Ok(())
}
