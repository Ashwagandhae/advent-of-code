
import java.util.*;

class Floor {
  boolean[][] terrain;
  int highestRock;
  RockLoop rockLoop;
  InstructLoop instructLoop;

  // no height because it increases
  Floor(int width, int height, RockLoop rockLoop, InstructLoop instructLoop) {
    this.terrain = new boolean[width + 2][height];
    // set bottom to rock
    for (int x = 0; x < width + 2; x++) {
      terrain[x][height - 1] = true;
    }
    // set left and right to rock
    for (int y = 0; y < height; y++) {
      terrain[0][y] = true;
      terrain[width + 1][y] = true;
    }
    highestRock = height - 1;
    this.rockLoop = rockLoop;
    this.instructLoop = instructLoop;

  }

  void dropRock() {
    Rock rock = rockLoop.next();
    // start at three because wall
    int rockX = 3;
    int rockY = highestRock - rock.height - 3;
    // System.out.println("Rock starts falling from " + rockX + ", " + rockY);
    // if (rockY < 0) {
    // extendTerrain(rockY);
    // }
    while (true) {
      rockX = hotAirJet(rock, rockX, rockY);
      if (canFallDown(rock, rockX, rockY)) {
        rockY++;
        // System.out.println("Rock falls down");
      } else {
        absorbRock(rock, rockX, rockY);
        // System.out.println("Rock falls down and comes to rest");
        break;
      }
    }

  }

  void printTerrain() {
    System.out.println("");
    for (int y = 0; y < terrain[0].length; y++) {
      for (int x = 0; x < terrain.length; x++) {
        if (terrain[x][y]) {
          System.out.print("#");
        } else {
          System.out.print(".");
        }
      }
      System.out.println();
    }
  }

  void extendTerrain(int y) {
    boolean[][] newTerrain = new boolean[terrain.length][terrain[0].length - y];
    for (int x = 0; x < terrain.length; x++) {
      for (int newY = 0; newY < terrain[0].length; newY++) {
        newTerrain[x][newY] = terrain[x][newY - y];
      }
    }
    terrain = newTerrain;
  }

  int hotAirJet(Rock rock, int x, int y) {
    char instruction = instructLoop.next();
    int originalX = x;
    if (instruction == '<') {
      x--;
      // System.out.println("Jet of gas blows left");
    } else if (instruction == '>') {
      x++;
      // System.out.println("Jet of gas blows right");
    }
    if (rockIsCollided(rock, x, y)) {
      // System.out.println(" --But nothing happens");
      return originalX;
    }
    return x;

  }

  boolean canFallDown(Rock rock, int x, int y) {
    if (rockIsCollided(rock, x, y + 1)) {
      return false;
    }
    return true;
  }

  void absorbRock(Rock rock, int x, int y) {
    for (int rockY = 0; rockY < rock.height; rockY++) {
      for (int rockX = 0; rockX < rock.width; rockX++) {
        if (rock.shape[rockY][rockX]) {
          terrain[x + rockX][y + rockY] = true;
        }
      }
    }
    updateHighestRock();
  }

  void updateHighestRock() {
    // get rock with highest y
    for (int y = 0; y < terrain[0].length; y++) {
      // exclude walls
      for (int x = 1; x < terrain.length - 1; x++) {
        if (terrain[x][y]) {
          if (y < highestRock) {
            highestRock = y;
          }
        }
      }
    }
  }

  boolean rockIsCollided(Rock rock, int x, int y) {
    for (int rockY = 0; rockY < rock.height; rockY++) {
      for (int rockX = 0; rockX < rock.width; rockX++) {
        if (rock.shape[rockY][rockX]) {
          if (terrain[x + rockX][y + rockY]) {
            return true;
          }
        }
      }
    }
    return false;
  }

  int getHighestRock() {
    return terrain[0].length - highestRock;
  }
}

class Rock {
  boolean[][] shape;
  int width;
  int height;

  Rock(String[] shape) {
    this.shape = new boolean[shape.length][shape[0].length()];
    for (int y = 0; y < shape.length; y++) {
      for (int x = 0; x < shape[y].length(); x++) {
        this.shape[y][x] = shape[y].charAt(x) == '#';
      }
    }
    this.width = shape[0].length();
    this.height = shape.length;

  }
}

class RockLoop {
  String[][] shapes;
  int index;

  RockLoop() {
    shapes = new String[5][];
    shapes[0] = new String[] {
        "####",
    };
    shapes[1] = new String[] {
        ".#.",
        "###",
        ".#."
    };
    shapes[2] = new String[] {
        "..#",
        "..#",
        "###"
    };
    shapes[3] = new String[] {
        "#",
        "#",
        "#",
        "#"
    };
    shapes[4] = new String[] {
        "##",
        "##",
    };
    index = 0;

  }

  Rock next() {
    Rock rock = new Rock(shapes[index]);
    index = (index + 1) % shapes.length;
    return rock;
  }
}

class InstructLoop {
  String instructions;
  int index;

  InstructLoop(String instructions) {
    this.instructions = instructions;
    index = 0;
  }

  char next() {
    char instruction = instructions.charAt(index);
    index = (index + 1) % instructions.length();
    return instruction;
  }

}

public class Main {
  public static void main(String[] args)
      throws Exception {
    // create file object
    java.io.File file = new java.io.File("../data/17.txt");
    // create scanner object
    Scanner scanner = new Scanner(file);
    // read file as string
    String input = scanner.nextLine();
    // close scanner
    scanner.close();

    InstructLoop instructLoop = new InstructLoop(input);
    RockLoop rockLoop = new RockLoop();
    Floor floor = new Floor(7, 2022 * 10, rockLoop, instructLoop);
    for (int i = 0; i < 2022; i++) {
      floor.dropRock();
      if (i % 100 == 0) {
        // System.out.println("Rock #" + i);
      }
    }
    int answer = floor.getHighestRock() - 1;

    System.out.println(answer);

  }
}