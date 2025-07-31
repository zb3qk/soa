pub fn list_agents(filter: Option<String>) {
    println!(
        "Fetching list of agents{}...",
        match &filter {
            Some(f) => format!(" (filter: {})", f),
            None => String::from(""),
        }
    );
    // Simulated output
    println!("agent-01\tplanner\tRunning");
    println!("agent-02\tingestor\tIdle");
}

pub fn status_agent(id: &str) {
    println!("Fetching status for agent: {}", id);
    // Simulated JSON output
    println!(
        "{}",
        serde_json::json!({
            "id": id,
            "type": "ingestor",
            "state": "Running",
            "cpu": "78%",
            "mem": "312MB",
            "last_heartbeat": "2025-07-28T16:55:10Z"
        })
            .to_string()
    );
}

pub fn send_command(id: &str, action: &str) {
    println!("Sending action '{}' to agent '{}'", action, id);
    // Simulated response
    println!("✅ Command sent.");
}

pub fn list_tasks() {
    println!("Listing tasks...");
    println!("task-001\tagent-01\tIn Progress");
    println!("task-002\tagent-02\tQueued");
}

pub fn cancel_task(id: &str) {
    println!("Cancelling task: {}", id);
    println!("✅ Task {} cancelled.", id);
}

pub fn assign_task(task_id: &str, to: &str) {
    println!("Assigning task '{}' to agent '{}'", task_id, to);
    println!("✅ Assignment complete.");
}

pub enum OutputFormat {
    Json,
    Toml,
    Yaml,
    Csv,
}

pub fn config_add(output: Option<OutputFormat>) {
    let args = crate::agentctl::config::add::AddArgs { output };
    crate::agentctl::config::add::add(&args);
}

pub fn config_show(output: Option<OutputFormat>) {
    let args = crate::agentctl::config::show::ShowArgs { output };
    crate::agentctl::config::show::show(&args);
}

pub fn env_show(output: Option<OutputFormat>) {
    let args = crate::agentctl::env::show::ShowArgs { output };
    crate::agentctl::env::show::show(&args);
}

pub fn env_set(profile_id: &str, output: Option<OutputFormat>) {
    let args = crate::agentctl::env::set::SetArgs {
        profile_id: profile_id.to_string(),
        output,
    };
    crate::agentctl::env::set::set(&args);
}

pub fn describe(id: &str, output: Option<OutputFormat>) {
    let args = crate::agentctl::describe::DescribeArgs {
        id: id.to_string(),
        output,
    };
    crate::agentctl::describe::describe(&args);
}

pub fn deploy(id: &str, output: Option<OutputFormat>) {
    let args = crate::agentctl::deploy::DeployArgs {
        id: id.to_string(),
        output,
    };
    crate::agentctl::deploy::deploy(&args);
}

pub fn invoke(id: &str, input: &str, output: Option<OutputFormat>) {
    let args = crate::agentctl::invoke::InvokeArgs {
        id: id.to_string(),
        input: input.to_string(),
        output,
    };
    crate::agentctl::invoke::invoke(&args);
}

pub fn list(id: &str, output: Option<OutputFormat>) {
    let args = crate::agentctl::list::ListArgs {
        id: id.to_string(),
        output,
    };
    crate::agentctl::list::list(&args);
}
