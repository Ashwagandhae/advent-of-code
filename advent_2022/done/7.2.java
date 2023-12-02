
import java.util.*;

class FileObj {
  public String name;
  public long score;

  public FileObj(String name, long score) {
    this.name = name;
    this.score = score;
  }

  public long getScore() {
    return this.score;
  }
}

class Folder {
  public String name;
  public ArrayList<Folder> folders;
  public ArrayList<FileObj> files;

  public Folder(String name) {
    this.name = name;
    this.folders = new ArrayList<Folder>();
    this.files = new ArrayList<FileObj>();
  }

  public void addFolder(Folder folder) {
    this.folders.add(folder);
  }

  public void addFile(FileObj file) {
    this.files.add(file);
  }

  public long getScore(ArrayList<Long> arr) {
    long score = 0;

    for (FileObj file : this.files) {
      score += file.getScore();
    }
    for (Folder folder : this.folders) {
      score += folder.getScore(arr);

    }
    arr.add(score);
    return score;
  }

  public void print() {
    System.out.println("folder " + this.name);
    for (FileObj file : this.files) {
      System.out.println("file  " + file.name + " " + file.score);
    }
    for (Folder folder : this.folders) {
      folder.print();
    }
    System.out.println("/folder " + this.name);
  }

}

public class Main {
  public static void main(String[] args)
      throws Exception {
    // create file object
    java.io.File file = new java.io.File("../data/7.txt");
    // create scanner object
    Scanner scanner = new Scanner(file);
    // read file
    Folder root = new Folder("");

    ArrayList<String> currentPath = new ArrayList<String>();

    // loop through lines
    while (scanner.hasNextLine()) {
      // read line
      String line = scanner.nextLine();
      // split line
      if (line.startsWith("$")) {
        line = line.substring(2);
        // get first two chars
        // check if first two chars are digits
        if (line.startsWith("cd")) {
          line = line.substring(3);
          if (line.equals("..")) {
            currentPath.remove(currentPath.size() - 1);
          } else if (line.equals("/")) {
            currentPath = new ArrayList<String>();
          } else {
            currentPath.add(line);
          }
        } else {
          // do nothing
        }
      } else {
        // get currentPath
        Folder currentFolder = root;
        for (String folderName : currentPath) {
          for (Folder folder : currentFolder.folders) {
            if (folder.name.equals(folderName)) {
              currentFolder = folder;
              break;
            }
          }

        }
        System.out.println("currentPath: " + currentPath);
        System.out.println("currentFolder: " + currentFolder.name);
        if (line.startsWith("dir")) {
          // add folder
          String folderName = line.substring(4);
          Folder folder = new Folder(folderName);
          currentFolder.addFolder(folder);
        } else {
          // add file
          String[] parts = line.split(" ");
          String fileName = parts[1];
          long score = Integer.parseInt(parts[0]);
          currentFolder.addFile(new FileObj(fileName, score));
        }

      }
    }
    // get size of each directory recursifly
    root.print();
    ArrayList<Long> arr = new ArrayList<Long>();
    long spaceUsed = root.getScore(arr);

    long spaceNeeded = 30000000;
    long totalSpace = 70000000;
    long currentSpace = totalSpace - spaceUsed;
    long minDeleteSpace = spaceNeeded - currentSpace;
    System.out.println("mindeletespace" + minDeleteSpace);
    ArrayList<Long> deleteCandidates = new ArrayList<Long>();
    for (long num : arr) {
      if (num >= minDeleteSpace) {
        deleteCandidates.add(num);
      }
    }
    // get min
    long min = deleteCandidates.get(0);
    for (long num : deleteCandidates) {
      if (num < min) {
        min = num;
      }
    }
    // print min
    System.out.println(min);

  }
}