public class Command {
    private CommandType type;
    private int value;

    public Command(String input) {
        String[] combo = input.split(" ");
        switch (combo[0]) {
            case "forward":
            this.type = CommandType.Forward;
            this.value = Integer.parseInt(combo[1]);
            break;
            case "up":
            this.type = CommandType.Up;
            this.value = -(Integer.parseInt(combo[1]));
            break;
            case "down":
            this.type = CommandType.Down;
            this.value = Integer.parseInt((combo[1]));
            break;
        }
    }

    public CommandType getCommandType() {
        return this.type;
    }

    public int getValue() {
        return this.value; 
    }
}
