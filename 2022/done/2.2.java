
import java.io.*;
import java.util.*;

public class Main {
  public static void main(String[] args)
      throws Exception {
    // create file object
    File file = new File("../data/2.txt");
    // create scanner object
    Scanner scanner = new Scanner(file);
    // read file
    int score = 0;
    while (scanner.hasNextLine()) {
      String line = scanner.nextLine();
      String[] tokens = line.split(" ");
      String opp = tokens[0].replace("A", "X").replace("B", "Y").replace("C", "Z");
      String instruct = tokens[1];
      if (instruct.equals("X")) {
        // lose
        if (opp.equals("X")) {
          score += 3;
        } else if (opp.equals("Y")) {
          score += 1;
        } else if (opp.equals("Z")) {
          score += 2;
        }

      } else if (instruct.equals("Y")) {
        // tie
        score += 3;
        if (opp.equals("X")) {
          score += 1;
        } else if (opp.equals("Y")) {
          score += 2;
        } else if (opp.equals("Z")) {
          score += 3;
        }
      } else if (instruct.equals("Z")) {
        // win
        score += 6;
        if (opp.equals("X")) {
          score += 2;
        } else if (opp.equals("Y")) {
          score += 3;
        } else if (opp.equals("Z")) {
          score += 1;
        }
      }

    }
    // close scanner
    scanner.close();
    // print score
    System.out.println(score);

  }
}