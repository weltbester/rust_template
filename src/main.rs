use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
	println!("Hello, {}!", args.name);
    }

// The responsibilities that remain in the main function after this process should be limited to the following:

// - Calling the command line parsing logic with the argument values
// - Setting up any other configuration
// - Calling a run function in lib.rs
// - Handling the error if run returns an error    

}

// cargo generate username-on-github/mytemplate
// # is the same as
// cargo generate gh:username-on-github/mytemplate
// # is the same as
// cargo generate --git https://github.com/username-on-github/mytemplate.git

// # for gitlab.com
// cargo generate gl:username-on-gitlab/mytemplate # translates to https://gitlab.com/username-on-gitlab/mytemplate.git
// # or for bitbucket.org
// cargo generate bb:username-on-bitbucket/mytemplate # translates to https://bitbucket.org/username-on-bitbucket/mytemplate.git
// # or for github.com
// cargo generate gh:username-on-github/mytemplate # translates to https://github.com/username-on-github/mytemplate.git
// # or for git.sr.ht
// cargo generate sr:username-on-sourcehut/mytemplate # translates to https://git.sr.ht/~username-on-sourcehut/mytemplate (note the tilde)

// git clone https://github.com/username-on-github/mytemplate.git $HOME/mytemplate # Clone any template
// cargo generate --path $HOME/mytemplate # Use it locally
