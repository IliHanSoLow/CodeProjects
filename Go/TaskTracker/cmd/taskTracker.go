package cmd

import (
	"encoding/gob"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"
)

// !!! FEHLER WAHRSCHEINLICH BEI LESEN ODER SCHREIBEN ZU BINARY
// ? Hoffentlich funktioniert das so

var currentUser user

func InitUser(name string) {
	storeArray(currentUser.List, currentUser.Name)
	if _, err := os.Stat("taskTracer/" + name); os.IsNotExist(err) {
		storeArray(nil, name)
		currentUser = user{name, []item{}}
		currentUser.List = readArray(name)
		return
	}
	currentUser = user{name, []item{}}
	currentUser.List = readArray(name)
	currentUser.List = readArray(name)
}

func newTask(title string, priority int, description string) {
	nTask := item{title, len(currentUser.List), priority, description, false, time.Now()}
	currentUser.List = append(currentUser.List, nTask)
}

func printTasks() {
	fmt.Printf("palce\tname\tpriority\tTime\t\t\t\tDescription\n")
	for _, value := range currentUser.List {
		fmt.Printf("%v.\t%v \t%v  \t\t%v\t%v\tCompleted: %v\n", value.Place, value.Title, value.Priority, value.CTime.Round(time.Minute).String(), value.Description, value.Completed)
	}
	println()
}

func editTask(place int, newtitle string, newpriority int, newdescription string) {
	completed := currentUser.List[place].Completed
	time := currentUser.List[place].CTime
	currentUser.List[place] = item{newtitle, place, newpriority, newdescription, completed, time}
}

func completeTask(place int) {
	currentUser.List[place].toggleCompletion()
}

func deleteTask(place int) {
	currentUser.List = append(currentUser.List[:place], currentUser.List[place+1:]...)
}

func sortBy(sortalg int) {
	switch sortalg {
	case Priority:
		currentUser.List = quicksortPrioStart(currentUser.List)
		resetPlace()
	case Title:
		currentUser.List = quicksortTitleStart(currentUser.List)
		resetPlace()
	case Time:
		currentUser.List = quicksortTimeStart(currentUser.List)
		resetPlace()
	case Completion:
		currentUser.List = quicksortCompletionStart(currentUser.List)
		resetPlace()
	}
}

func resetPlace() {
	for i := 0; i < len(currentUser.List); i++ {
		currentUser.List[i].Place = i
	}
}

func quicksortPrioStart(arr []item) []item {
	return quickSortPrio(arr, 0, len(arr)-1)
}

func quickSortPrio(arr []item, low, high int) []item {
	if low < high {
		var p int
		arr, p = partitionPrio(arr, low, high)
		arr = quickSortPrio(arr, low, p-1)
		arr = quickSortPrio(arr, p+1, high)
	}
	return arr
}

func partitionPrio(arr []item, low, high int) ([]item, int) {
	pivot := arr[high].Priority
	i := low
	for j := low; j < high; j++ {
		if arr[j].Priority < pivot {
			arr[i], arr[j] = arr[j], arr[i]
			i++
		}
	}
	arr[i], arr[high] = arr[high], arr[i]
	return arr, i
}

func quicksortTitleStart(arr []item) []item {
	return quickSortTitle(arr, 0, len(arr)-1)
}

func quickSortTitle(arr []item, low, high int) []item {
	if low < high {
		var p int
		arr, p = partitionTitle(arr, low, high)
		arr = quickSortTitle(arr, low, p-1)
		arr = quickSortTitle(arr, p+1, high)
	}
	return arr
}

func partitionTitle(arr []item, low, high int) ([]item, int) {
	pivot := arr[high].Title
	i := low
	for j := low; j < high; j++ {
		if strings.Compare(strings.ToLower(arr[j].Title), strings.ToLower(pivot)) == -1 {
			arr[i], arr[j] = arr[j], arr[i]
			i++
		}
	}
	arr[i], arr[high] = arr[high], arr[i]
	return arr, i
}

func quicksortTimeStart(arr []item) []item {
	return quickSortTime(arr, 0, len(arr)-1)
}

func quickSortTime(arr []item, low, high int) []item {
	if low < high {
		var p int
		arr, p = partitionTime(arr, low, high)
		arr = quickSortTime(arr, low, p-1)
		arr = quickSortTime(arr, p+1, high)
	}
	return arr
}

func partitionTime(arr []item, low, high int) ([]item, int) {
	pivot := arr[high].CTime
	i := low
	for j := low; j < high; j++ {
		if arr[j].CTime.Before(pivot) {
			arr[i], arr[j] = arr[j], arr[i]
			i++
		}
	}
	arr[i], arr[high] = arr[high], arr[i]
	return arr, i
}

func quicksortCompletionStart(arr []item) []item {
	return quickSortCompletion(arr, 0, len(arr)-1)
}

func quickSortCompletion(arr []item, low, high int) []item {
	if low < high {
		var p int
		arr, p = partitionCompletion(arr, low, high)
		arr = quickSortCompletion(arr, low, p-1)
		arr = quickSortCompletion(arr, p+1, high)
	}
	return arr
}

func partitionCompletion(arr []item, low, high int) ([]item, int) {
	pivot := arr[high].Completed
	i := low
	for j := low; j < high; j++ {
		if arr[j].Completed == false && pivot == true {
			arr[i], arr[j] = arr[j], arr[i]
			i++
		}
	}
	arr[i], arr[high] = arr[high], arr[i]
	return arr, i
}

func storeArray(arr []item, userName string) {
	p := "taskTracer/" + userName + "/array.dat"
	// Open the file for writing
	if err := os.MkdirAll(filepath.Dir(p), 0770); err != nil {
		fmt.Println("ERROR FILPATH COULD NOT BE CREATED")
		return
	}
	file, err := os.Create(p)
	if err != nil {
		fmt.Println("Error creating file:", err)
		return
	}
	defer file.Close()

	// Create an encoder
	encoder := gob.NewEncoder(file)

	// Encode and write the array to the file
	err = encoder.Encode(arr)
	if err != nil {
		fmt.Println("Error encoding array:", err)
		return
	}

	// fmt.Println("Array has been stored in 'array.dat'")
}

func readArray(userName string) (arr []item) {
	// Open the file for reading
	file, err := os.Open("taskTracer/" + userName + "/array.dat")
	if err != nil {
		fmt.Println("Error opening file:", err)
		return
	}
	defer file.Close()

	// Create a decoder
	decoder := gob.NewDecoder(file)

	// Decode the array from the file
	err = decoder.Decode(&arr)
	if err != nil {
		fmt.Println("Error decoding array:", err)
		return
	}

	// fmt.Println("Array read from 'array.dat':", arr)
	return
}

const (
	Title      int = 0
	Completion int = 1
	Priority   int = 2
	Time       int = 3
)

type item struct {
	Title       string
	Place       int
	Priority    int
	Description string
	Completed   bool
	CTime       time.Time
}

func (self *item) toggleCompletion() {
	self.Completed = !self.Completed
}

type user struct {
	Name string
	List []item
}
