import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;

public class Dive {
    public static void main(String[] args) {
        if (args.length == 0) return;
        int depth = 0, horizontal = 0, aim = 0;
        try {
            BufferedReader bf = new BufferedReader(new FileReader(args[0]));
            ArrayList<Command> commands = new ArrayList<>();
            String read;
            while ((read = bf.readLine()) != null) {
                commands.add(new Command(read));
            }

            for (Command c : commands) {
                CommandType type = c.getCommandType();
                if (type == CommandType.Forward) {
                    final int value = c.getValue();
                    horizontal += value;
                    depth += value * aim;
                } else {
                    aim += c.getValue();
                }
            }

            final int result = horizontal * depth;
            System.out.println("Position and depth (horizontal * depth): " + result);

            bf.close();
        } catch (IOException e) {
            System.err.print(e);
        } 
    }
}