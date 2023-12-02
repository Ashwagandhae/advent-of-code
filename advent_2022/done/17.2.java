
import java.util.*;

class Floor {
  boolean[][] terrain;
  int highestRock;
  int highestRocks[];
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
    this.highestRocks = new int[width];
    this.highestRock = height;
    for (int x = 0; x < width; x++) {
      highestRocks[x] = height;
    }
    this.rockLoop = rockLoop;
    this.instructLoop = instructLoop;

  }

  void dropRock() {
    Rock rock = rockLoop.next();
    // start at three because wall
    int rockX = 3;
    int rockY = highestRock - rock.height - 3;
    if (rockY < 0) {
      extendTerrain(100);
      rockY = highestRock - rock.height - 3;
    }

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

  boolean highestRockFlat() {
    for (int x = 0; x < terrain.length; x++) {
      if (!terrain[x][highestRock]) {
        return false;
      }
    }
    return true;
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

  void extendTerrain(int amount) {
    boolean[][] newTerrain = new boolean[terrain.length][terrain[0].length + amount];
    for (int y = 0; y < terrain[0].length; y++) {
      for (int x = 0; x < terrain.length; x++) {
        newTerrain[x][y + amount] = terrain[x][y];
      }
    }
    // set side walls
    for (int y = 0; y < terrain.length; y++) {
      newTerrain[0][y] = true;
      newTerrain[newTerrain.length - 1][y] = true;
    }

    terrain = newTerrain;
    // update highest rock
    highestRock += amount;
    // update highest rocks
    for (int x = 0; x < highestRocks.length; x++) {
      highestRocks[x] += amount;
    }
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
    for (int y = 0; y < highestRock; y++) {
      // exclude walls
      for (int x = 1; x < terrain.length - 1; x++) {
        if (terrain[x][y]) {
          if (y < this.highestRock) {
            this.highestRock = y;
          }
        }
      }
    }
    // get highest rock for each column
    for (int x = 1; x < terrain.length - 1; x++) {
      for (int y = 0; y < highestRocks[x - 1]; y++) {
        if (terrain[x][y]) {
          if (y < highestRocks[x - 1]) {
            highestRocks[x - 1] = y;
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
    Floor floor = new Floor(7, 100000 * 5, rockLoop, instructLoop);
    // create dictionary for cycle finder
    HashMap<String, String> cycleFinder = new HashMap<String, String>();
    int cycleStart = 0;
    int cycleLength = 0;
    int cycleRockHeight = 0;
    for (int i = 0; i < 100000; i++) {
      floor.dropRock();
      // copy floor.highestRocks and subtract highestRock from each element
      int[] highestRocks = Arrays.copyOf(floor.highestRocks, floor.highestRocks.length);
      for (int j = 0; j < highestRocks.length; j++) {
        highestRocks[j] -= floor.highestRock;
      }
      // convert instructLoop, rockLoop, and floor.highestRocks to string
      String key = instructLoop.index + " " + rockLoop.index + " " + Arrays.toString(highestRocks);
      // check if key exists
      if (cycleFinder.containsKey(key)) {
        // convert string key to i and highestRock
        int cycleIndex = Integer.parseInt(cycleFinder.get(key).split(" ")[0]);
        int cycleHighestRock = Integer.parseInt(cycleFinder.get(key).split(" ")[1]);

        cycleStart = cycleIndex;
        cycleLength = i - cycleIndex;
        cycleRockHeight = floor.getHighestRock() - cycleHighestRock;

        break;
      } else {
        // set value to i and getHighestRock
        cycleFinder.put(key, i + " " + floor.getHighestRock());
      }
      // print i every 100
      if (i % 100 == 0) {
        System.out.println(key);
      }
    }
    System.out.println("cycleStart: " + cycleStart);
    System.out.println("cycleLength: " + cycleLength);
    System.out.println("cycleRockHeight: " + cycleRockHeight);
    long iterations = 1_000_000_000_000L - cycleStart;
    long rockHeight = 0;
    // first get rock height at cycleStart
    // reset floor
    instructLoop = new InstructLoop(input);
    rockLoop = new RockLoop();
    floor = new Floor(7, 100000 * 5, rockLoop, instructLoop);
    for (int i = 0; i < cycleStart; i++) {
      floor.dropRock();
    }
    int beforeStartHeight = floor.getHighestRock();
    rockHeight = beforeStartHeight;
    // then add how many cycles fit in iterations
    rockHeight += (iterations / cycleLength) * cycleRockHeight;

    // then add rock height at cycleStart + iterations % cycleLength
    // reset floor
    // idk why i have to add 5 to iterations % cycleLength but it works
    for (int i = 0; i < iterations % cycleLength + 5; i++) {
      floor.dropRock();
    }
    rockHeight += floor.getHighestRock() - beforeStartHeight;
    System.out.println("rockHeight: " + rockHeight);

  }
}