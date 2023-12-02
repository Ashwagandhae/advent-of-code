
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
      String me = tokens[1];
      System.out.println(opp + " " + me);
      if (opp.equals(me)) {
        score += 3;
        System.out.println("eq");
      } else if ((opp.equals("X") && me.equals("Z")) || (opp.equals("Y") && me.equals("X"))
          || (opp.equals("Z") && me.equals("Y"))) {
        score += 0;
        System.out.println("L");
      } else {
        System.out.println("win");

        score += 6;
      }
      if (me.equals("X")) {
        score += 1;
      } else if (me.equals("Y")) {
        score += 2;
      } else if (me.equals("Z")) {
        score += 3;
      }
    }
    // close scanner
    scanner.close();
    // print score
    System.out.println(score);

  }
}