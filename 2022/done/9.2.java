
import java.util.*;

class Rope {
  int x;
  int y;
  Rope parent;

  public Rope(int x, int y, Rope parent) {
    this.x = x;
    this.y = y;
    this.parent = parent;
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

  public void moveIfNeeded() {
    if (parent != null) {
      if (Math.abs(parent.x - x) > 1 || Math.abs(parent.y - y) > 1) {
        // move tail towards parent
        moveTowards(parent.x, parent.y);
      }
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

    ArrayList<Rope> ropes = new ArrayList<Rope>();
    Rope head = new Rope(size / 2, size / 2, null);
    ropes.add(head);
    Rope lastRope = head;
    for (int i = 1; i <= 8; i++) {
      Rope rope = new Rope(size / 2, size / 2, lastRope);
      ropes.add(rope);
      lastRope = rope;
    }
    Rope tail = new Rope(size / 2, size / 2, lastRope);
    ropes.add(tail);
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
        for (Rope rope : ropes) {
          rope.moveIfNeeded();
        }
        visited[tail.y][tail.x] = true;

      }

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