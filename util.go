package main

import "bufio"
import "io"
import "os"
import "fmt"
import "strings"

func ReadLines(path string) []string {

	file, err := os.Open(path)
  if err != nil {
      fmt.Println("Couldn't open file:",err)
      os.Exit(1)
  }
  defer file.Close()


  // Start reading from the file with a reader.
  reader := bufio.NewReader(file)
  ret := []string{}
  var line string
  for {
    line, err = reader.ReadString('\n')
    if err != nil {
    	if err == io.EOF {
    		break
    	} else {
	      fmt.Println("Failed with error:", err)
	      os.Exit(1)
    	}
    }

		ret = append(ret,strings.TrimSpace(line))
  }
  
	return ret
}
