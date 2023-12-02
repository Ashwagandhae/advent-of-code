
import java.util.*;

class Rope {
  int x;
  int y;

  public Rope(int x, int y) {
    this.x = x;
    this.y = y;
  }

  public void moveTo(int x, int y) {
    this.x = x;
    this.y = y;
  }

  public void moveTowards(int target_x, int target_y) {
    if (target_x > x) {
      x++;
    }
    if (target_x < x) {
      x--;
    }
    if (target_y > y) {
      y++;
    }
    if (target_y < y) {
      y--;
    }

  }
}

public class Main {
  public static void main(String[] args)
      throws Exception {
    // create file object
    java.io.File file = new java.io.File("../data/9.txt");
    // create scanner object
    Scanner scanner = new Scanner(file);
    int answer = 0;

    // make 2d sizexsize array filled with char .
    int size = 1000;
    boolean[][] visited = new boolean[size][size];
    for (int i = 0; i < size; i++) {
      for (int j = 0; j < size; j++) {
        visited[i][j] = false;
      }
    }

    Rope head = new Rope(size / 2, size / 2);
    Rope tail = new Rope(size / 2, size / 2);
    visited[tail.y][tail.x] = true;
    // loop through lines
    while (scanner.hasNextLine()) {
      // get line
      String line = scanner.nextLine();
      // split line into array
      String[] lineArray = line.split(" ");
      // get direction (R, L, U, D) and count
      char direction = lineArray[0].charAt(0);
      int count = Integer.parseInt(lineArray[1]);
      for (int l = 0; l < count; l++) {
        switch (direction) {
          case 'R':
            head.moveTo(head.x + 1, head.y);
            break;
          case 'L':
            head.moveTo(head.x - 1, head.y);
            break;
          case 'U':
            head.moveTo(head.x, head.y - 1);
            break;
          case 'D':
            head.moveTo(head.x, head.y + 1);
            break;
        }
        // if x or y diff larger than 1
        if (Math.abs(head.x - tail.x) > 1 || Math.abs(head.y - tail.y) > 1) {
          // move tail towards head
          tail.moveTowards(head.x, head.y);
        }
        visited[tail.y][tail.x] = true;

      }
      // print visited array

    }
    // close Scanner
    scanner.close();

    // count visited
    for (int i = 0; i < size; i++) {
      for (int j = 0; j < size; j++) {
        if (visited[i][j]) {
          answer++;
        }
      }
    }

    System.out.println(answer);

  }
}