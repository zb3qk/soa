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