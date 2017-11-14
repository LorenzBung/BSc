use procinfo::pid;

/// Datenstruktur für einen Prozess.
pub struct Process {
    name: String,
    pid: u32,
    ppid: u32,
}

impl Process {
    /// Erstellt eine Prozess-Datenstruktur aus procinfo::Stat.
    pub fn new(with_pid: u32) -> Self {
        if let Ok(stat) = pid::stat(with_pid) {
            Process {
                name: stat.command,
                pid: stat.pid,
                ppid: stat.ppid,
            }
        } else {
            panic!("Internal Error: Process not found")
        }
    }

    /// Erstellt eine Prozess-Datenstruktur aus procinfo::Stat.
    pub fn me() -> Self {
        if let Ok(my_pid) = pid::stat_self() {
            Process::new(my_pid.pid)
        } else {
            panic!("Internal Error: I don't have a PID but I am running.")
        }
    }

    /// Prüft ob das Prozess-Struct ein Elternprozess besitzt.
    pub fn has_parent(&self) -> bool {
        self.ppid != 0
    }

    /// Gibt den Elternprozess zurück.
    pub fn parent(&self) -> Self {
        Process::new(self.ppid)
    }

    /// Prüft ob das Prozess-Struct einen (entfernten) Elternprozess mit dem übergebenen pid hat.
    pub fn has_parent_with_pid(&self, pid: u32) -> bool {
        if self.pid == pid {
            return true;
        }

        if self.has_parent() {
            return self.parent().has_parent_with_pid(pid);
        }

        false
    }

    /// Gibt über Rekursion über die Eltern eine Prozesskette aus.
    pub fn print_recursive(&self, to_pid: u32, output: &mut String) {

        if output.len() == 0 {
            *output = format!("{}({}){}", self.name, self.pid, output);
        } else {
            *output = format!("{}({})---{}", self.name, self.pid, output);
        }

        if self.has_parent() && self.pid != to_pid {
            self.parent().print_recursive(to_pid, output);
        }
    }
}

/// Geht von eigenem Prozess aus und gibt die Prozesskette bis zum übergebenem PID aus
/// und fängt mögliche Fehler ab.
pub fn print(pid: u32) -> bool {

    if let Err(_) = pid::stat(pid) {
        println!("Invalid PID");
        return false;
    }

    let my_proc = Process::me();

    if !my_proc.has_parent_with_pid(pid) {
        println!("This Process has no parent {}", pid);
        return false;
    }

    let mut output = String::new();
    my_proc.print_recursive(pid, &mut output);
    println!("{}", output);

    true
}
