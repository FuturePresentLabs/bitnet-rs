use std::collections::HashMap;
use rand::seq::IteratorRandom; // Import the trait for random selection

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("bitnet-rs/include/bitnet_model.h");

        unsafe fn run_cli(args: Vec<String>) -> i32;
    }
}

#[derive(Debug)]
struct LlamaConfig {
    model_path: String,
    batch_size: i32,
    context_size: i32,
    threads: i32,
    ngl: i32,
    num_tokens: i32,
    temperature: i32,
    prompt: String,
}

impl LlamaConfig {
    // Create a new instance with default values
    fn new() -> Self {
        Self {
            model_path: "BitNet/models/Llama3-8B-1.58-100B-tokens/ggml-model-i2_s.gguf".to_string(),
            // model_path: "BitNet/models/bitnet_b1_58-3B/ggml-model-tl1.gguf".to_string(),
            batch_size: 2,
            context_size: 2048,
            threads: 8,
            ngl: 0,
            num_tokens: 50,
            temperature: 0,
            prompt: "Once upon a time I told a great story. Remind me of it".to_string(),
        }
    }

    // Convert config fields to `argv` format for FFI calls
    fn to_argv(&self) -> Vec<String> {
        vec![
            "llama-cli".to_string(),
            "-b".to_string(), self.batch_size.to_string(),
            "-c".to_string(), self.context_size.to_string(),
            "-t".to_string(), self.threads.to_string(),
            "-n".to_string(), self.num_tokens.to_string(),
            "-ngl".to_string(), self.ngl.to_string(),
            "--temp".to_string(), self.temperature.to_string(),
            "-m".to_string(), self.model_path.clone(),
            "-p".to_string(), self.prompt.clone(),
        ]
    }

    fn help(&self) -> Vec<String> {
        vec![
            "llama-cli".to_string(),
            "--help".to_string()
        ]
    }
}

fn help() {
    let args = LlamaConfig::new().help();
    unsafe {
        ffi::run_cli(args);
    }
}

// Run example that converts Vec<String> into CxxVector<CxxString>
fn run(config: &LlamaConfig) {
    // Get argv as Vec<String>
    let args = config.to_argv();

    // Call main_logic with argc and argv as CxxVector<CxxString>
    unsafe {
        ffi::run_cli(args);
    }
}

fn get_prompt() -> String {
    let prompts: HashMap<&str, &str> = [
        ("Creativity and Storytelling", 
         "Imagine you're a detective in a cyberpunk world where humans and AI coexist. Write the first page of your investigative journal as you start to uncover a conspiracy that blurs the lines between artificial and human intelligence."),

         ("Logical Reasoning", 
          "Explain the following: A man standing in front of a painting in a museum says, 'Brothers and sisters, I have none, but that man's father is my father's son.' Who is the person in the painting?"),

          ("Conversational and Empathy Skills", 
           "Imagine you’re a psychologist helping someone who feels stuck in life. They're unsure about their career, feel disconnected socially, and are questioning their goals. Walk through the first 10 minutes of your conversation with them."),

           ("Complex Instructions and Sequential Reasoning", 
            "Explain how to set up a home automation system that controls lights, temperature, and music preferences based on time and personal mood data collected from wearable devices. Make it understandable to someone with basic tech skills."),

            ("Advanced Coding Skills", 
             "Write a Python script that uses OpenCV to detect faces in an image, then applies a blur effect to anonymize the faces. Add comments explaining each step in the code."),

             ("Analytical Comparison", 
              "Compare and contrast the economic principles of Adam Smith and John Maynard Keynes, providing one modern example for each to demonstrate their influence on today's economies."),

              ("Linguistic and Cultural Understanding", 
               "Translate this passage from English to Japanese, considering nuances in formal vs. informal speech, and explain why you chose the level of formality you did: 'In the end, all we have are memories. Make them worth remembering.'"),

               ("Humor and Wordplay", 
                "Invent three original riddles that each contain a hidden pun. One should be about nature, another about technology, and the third about space travel."),

                ("History and Futurism", 
                 "If Napoleon Bonaparte were resurrected today and could advise a current political leader, who might he choose and what would his primary piece of advice be based on his experiences in the Napoleonic Wars?"),

                 ("Multimodal or Sensory Description", 
                  "Describe the aroma, sounds, and visuals of a bustling night market in Bangkok. Incorporate specific food smells, vendor sounds, and cultural elements to create a vivid mental image."),

                  ("Moral Dilemma and Ethical Reasoning", 
                   "A self-driving car must choose between two paths: one where it risks the passenger's safety and another where it endangers pedestrians. Explain the factors that should be weighed in programming its decision-making algorithm and discuss potential societal implications."),

                   ("Product and Business Ideation", 
                    "You are an entrepreneur developing a new productivity app. Describe its key features, ideal target audience, and how it would differentiate itself from competitors. Bonus: suggest a catchy name for the app."),

                    ("Science Fiction Scenario", 
                     "Humanity has discovered a massive underground ocean on a nearby moon, where unknown forms of life exist. You’re a scientist on the first exploratory mission. Write a report on your initial findings and describe the environmental conditions, life forms, and potential for further exploration."),

                     ("Historical Recreation", 
                      "Imagine you're the head chef at a royal feast in the court of Cleopatra. Describe the dishes you'd serve using ingredients and techniques available in Ancient Egypt and detail how you'd present them to impress the queen and her guests."),

                      ("Financial Analysis and Investment Advice", 
                       "You’re a financial analyst advising a client on the advantages and disadvantages of investing in green energy stocks versus traditional energy. Provide a balanced overview, addressing short- and long-term risks and benefits."),
                      ].iter().cloned().collect();

    let mut rng = rand::thread_rng();

    let p = prompts.iter().choose(&mut rng).unwrap();
    println!("Using prompt: {}", p.0);
    return format!("{}\nAnswer:", p.1);
}

fn main() {
    let mut config = LlamaConfig::new();

    // config.prompt = get_prompt();
    config.prompt = "1+1+11=? Answer:".to_string();

    run(&config);
}
