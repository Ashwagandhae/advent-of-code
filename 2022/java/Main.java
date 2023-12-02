
import java.util.*;

class Pos {
  int x;
  int y;

  public Pos(int x, int y) {
    this.x = x;
    this.y = y;
  }

  public Pos(Pos pos) {
    this.x = pos.x;
    this.y = pos.y;
  }

  public Pos move(int dx, int dy) {
    return new Pos(x + dx, y + dy);
  }

  public boolean equals(Pos pos) {
    return this.x == pos.x && this.y == pos.y;
  }

  public String toString() {
    return "(" + x + ", " + y + ")";
  }
}

class Terrain {
  char[][] map;

  public Terrain(int width, ArrayList<String> rawMap) {
    this.map = new char[rawMap.size()][width];
    // fill map with ' ';
    for (int i = 0; i < map.length; i++) {
      for (int j = 0; j < map[i].length; j++) {
        map[i][j] = ' ';
      }
    }
    // fill map with rawMap
    for (int i = 0; i < rawMap.size(); i++) {
      for (int j = 0; j < rawMap.get(i).length(); j++) {
        map[i][j] = rawMap.get(i).charAt(j);
      }
    }
  }

  public Pos getStartPos() {
    for (int i = 0; i < map[0].length; i++) {
      if (map[0][i] == '.') {
        return new Pos(i, 0);
      }
    }
    return null;
  }

  public Pos changePos(Pos pos, char direction, int amount) {
    for (int i = 0; i < amount; i++) {
      // print(pos);

      Pos newPos = null;
      switch (direction) {
        case 'u':
          newPos = pos.move(0, -1);
          if (isNullPos(newPos)) {
            newPos = getLastYPos(newPos.x);
          }
          break;
        case 'd':
          newPos = pos.move(0, 1);
          if (isNullPos(newPos)) {
            newPos = getFirstYPos(newPos.x);
          }
          break;
        case 'l':
          newPos = pos.move(-1, 0);
          if (isNullPos(newPos)) {
            newPos = getLastXPos(newPos.y);
          }
          break;
        case 'r':
          newPos = pos.move(1, 0);
          if (isNullPos(newPos)) {
            newPos = getFirstXPos(newPos.y);
          }
          break;
      }
      if (isWallPos(newPos)) {
        continue;
      }
      pos = newPos;
      // wait 100ms

    }
    return pos;
  }

  public void print(Pos pos) {
    try {
      Thread.sleep(300);
    } catch (InterruptedException e) {
      e.printStackTrace();
    }

    // print 10 x 10 area around pos
    for (int i = pos.y - 10; i < pos.y + 10; i++) {
      for (int j = pos.x - 10; j < pos.x + 10; j++) {
        if (i == pos.y && j == pos.x) {
          System.out.print("X");
        } else {
          // check if pos is out of bounds
          if (j < 0 || j >= map[0].length || i < 0 || i >= map.length) {
            System.out.print(" ");
            continue;
          }
          System.out.print(map[i][j]);
        }
      }
      System.out.println();
    }
  }

  public boolean isWallPos(Pos pos) {
    return map[pos.y][pos.x] == '#';
  }

  public boolean isNullPos(Pos pos) {
    // check if pos is out of bounds
    if (pos.x < 0 || pos.x >= map[0].length || pos.y < 0 || pos.y >= map.length) {
      return true;
    }
    return map[pos.y][pos.x] == ' ';

  }

  public Pos getFirstXPos(int y) {
    for (int i = 0; i < map[0].length; i++) {
      if (map[y][i] != ' ') {
        return new Pos(i, y);
      }
    }
    return null;
  }

  public Pos getLastXPos(int y) {
    for (int i = map[0].length - 1; i >= 0; i--) {
      if (map[y][i] != ' ') {
        return new Pos(i, y);
      }
    }
    return null;
  }

  public Pos getFirstYPos(int x) {
    for (int i = 0; i < map.length; i++) {
      if (map[i][x] != ' ') {
        return new Pos(x, i);
      }
    }
    return null;
  }

  public Pos getLastYPos(int x) {
    for (int i = map.length - 1; i >= 0; i--) {
      if (map[i][x] != ' ') {
        return new Pos(x, i);
      }
    }
    return null;
  }
}

public class Main {
  public static void main(String[] args)
      throws Exception {
    // create file object
    java.io.File file = new java.io.File("../data/22.txt");
    // create scanner object
    Scanner scanner = new Scanner(file);
    // define rawmap
    ArrayList<String> rawMap = new ArrayList<String>();
    // read file
    while (true) {
      String nextLine = scanner.nextLine();
      // if not empty
      if (nextLine.length() > 0) {
        rawMap.add(nextLine);
      } else {
        break;
      }
    }
    // get biggest width
    int width = 0;
    for (String line : rawMap) {
      if (line.length() > width) {
        width = line.length();
      }
    }
    // create terrain
    Terrain terrain = new Terrain(width, rawMap);
    // get start pos
    Pos pos = terrain.getStartPos();
    char direction = 'r';
    String instructRaw = scanner.nextLine();
    ArrayList<String> instructs = new ArrayList<String>();
    // split instruct into ints and chars
    String currentNum = "";
    for (int i = 0; i < instructRaw.length(); i++) {
      if (instructRaw.charAt(i) == 'L' || instructRaw.charAt(i) == 'R') {
        if (currentNum.length() > 0) {
          instructs.add(currentNum);
          currentNum = "";
        }
        instructs.add(instructRaw.charAt(i) + "");
      } else {
        currentNum += instructRaw.charAt(i);
      }
    }
    if (currentNum.length() > 0) {
      instructs.add(currentNum);
    }
    for (String instr : instructs) {
      // print instructions
      if (instr.equals("L")) {
        switch (direction) {
          case 'u':
            direction = 'l';
            break;
          case 'd':
            direction = 'r';
            break;
          case 'l':
            direction = 'd';
            break;
          case 'r':
            direction = 'u';
            break;
        }
      } else if (instr.equals("R")) {
        switch (direction) {
          case 'u':
            direction = 'r';
            break;
          case 'd':
            direction = 'l';
            break;
          case 'l':
            direction = 'u';
            break;
          case 'r':
            direction = 'd';
            break;
        }
      } else {
        int amount = Integer.parseInt(instr);
        pos = terrain.changePos(pos, direction, amount);
      }

    }
    int row = pos.y + 1;
    int col = pos.x + 1;
    int facing = 0;
    switch (direction) {
      case 'r':
        facing = 0;
        break;
      case 'd':
        facing = 1;
        break;
      case 'l':
        facing = 2;
        break;
      case 'u':
        facing = 3;
        break;
    }
    System.out.println("Row: " + row + ", Col: " + col + ", Facing: " + facing);
    System.out.println(1000 * row + 4 * col + facing);
    // close scanner
    scanner.close();

  }
}