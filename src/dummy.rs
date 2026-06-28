pub mod dummy {
    use crate::{CommandArgs, CommandParam};

    pub fn base(args: Vec<String>) {
        // Define all the arguments and params for this command
        let mut arguments: Vec<CommandArgs> = vec![];
        let mut params: Vec<CommandParam> = vec![];

        let help = CommandArgs {
            arg_name: "--help".to_string(),
            // is_required: false,
            // require_params: false,
            help_message: "Says 'hi <name>'".to_string(),
        };

        let name_arg: CommandParam = CommandParam {
            param_name: "name".to_string(),
            is_required: true,
        };

        // Add them to the list of accepted arguments
        arguments.push(help);
        params.push(name_arg);

        // let required_arg_total = args.iter().find(|x| {x})
    }
}
