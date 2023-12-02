
import java.util.*;

class Pos {
  int x;
  int y;

  Pos(int x, int y) {
    this.x = x;
    this.y = y;
  }

  // print
  public String toString() {
    return "(" + x + "," + y + ")";
  }
}

class Cave {
  char[][] terrain;
  int size;

  Cave(ArrayList<ArrayList<Pos>> paths, int size, int floor) {
    // create empty terrain
    this.size = size;
    terrain = new char[size][size];
    for (int i = 0; i < size; i++) {
      for (int j = 0; j < size; j++) {
        terrain[i][j] = '.';
      }
    }

    for (ArrayList<Pos> path : paths) {
      for (int i = 0; i < path.size() - 1; i++) {
        Pos start = path.get(i);
        Pos end = path.get(i + 1);
        System.out.println("filling line from " + start + " to " + end + "");
        fillLine(start, end, '#');
      }
    }
    fillLine(new Pos(0, floor), new Pos(size - 1, floor), '#');
  }

  void fillLine(Pos start, Pos end, char material) {

    int x = start.x;
    int y = start.y;
    int dx = end.x - start.x;
    int dy = end.y - start.y;
    int steps = Math.max(Math.abs(dx), Math.abs(dy));
    dx = dx / steps;
    dy = dy / steps;
    for (int i = 0; i <= steps; i++) {
      terrain[y][x] = material;
      x += dx;
      y += dy;
    }
  }

  boolean dropSand(Pos startPos) {
    int x = startPos.x;
    int y = startPos.y;
    while (!outOfBounds(x, y)) {
      if (isAir(x, y + 1)) {
        y++;
      } else if (isAir(x - 1, y + 1)) {
        y++;
        x--;
      } else if (isAir(x + 1, y + 1)) {
        y++;
        x++;
      } else {
        this.terrain[y][x] = 'o';
        if (x == startPos.x && y == startPos.y) {
          return false;
        }
        return true;
      }
    }
    throw new AbstractMethodError(null);
  }

  boolean outOfBounds(int x, int y) {
    return x < 0 || x >= size || y < 0 || y >= size;
  }

  boolean isAir(int x, int y) {
    return terrain[y][x] == '.';
  }

  void printTerrain() {
    for (int i = 0; i < 20; i++) {
      for (int j = 450; j < size; j++) {
        System.out.print(terrain[i][j]);
      }
      System.out.println();
    }
  }

}

public class Main {
  public static void main(String[] args)
      throws Exception {
    // create file object
    java.io.File file = new java.io.File("../data/14.txt");
    // create scanner object
    Scanner scanner = new Scanner(file);
    ArrayList<ArrayList<Pos>> paths = new ArrayList<ArrayList<Pos>>();
    // loop each line
    int largestY = 0;
    while (scanner.hasNextLine()) {
      String line = scanner.nextLine();
      String[] parts = line.split("->");
      ArrayList<Pos> path = new ArrayList<Pos>();
      for (String part : parts) {
        String[] coords = part.split(",");
        int x = Integer.parseInt(coords[0].strip());
        int y = Integer.parseInt(coords[1].strip());
        path.add(new Pos(x, y));
        if (y > largestY) {
          largestY = y;
        }
      }
      paths.add(path);
    }
    scanner.close();

    int size = 800;
    Cave cave = new Cave(paths, size, largestY + 2);
    int answer = 0;
    while (cave.dropSand(new Pos(500, 0))) {
      answer++;
    }

    System.out.println(answer + 1);

  }
}