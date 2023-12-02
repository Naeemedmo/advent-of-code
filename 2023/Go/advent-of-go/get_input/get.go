package get_input

import (
	"fmt"
	"io/ioutil"
	"net/http"
	"os"
)

type AoCDate struct {
	Year int
	Day  int
}

func BuildUrl(year int, day int) string {
	return fmt.Sprintf("https://adventofcode.com/%d/day/%d/input", year, day)
}

func buildCookie(cookie string) string {
	return fmt.Sprintf("session=%s", cookie)
}

func GetInput(date AoCDate) string {

	cookie := os.Getenv("AOC_SESSION_COOKIE")
	if cookie == "" {
		fmt.Println("Error: AOC_SESSION_COOKIE must be set!")
		os.Exit(1)
	}
	session_cookie := buildCookie(cookie)

	url := BuildUrl(date.Year, date.Day)

	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		fmt.Println("Error creating HTTP request:", err)
		os.Exit(1)
	}

	req.Header.Set("Cookie", session_cookie)

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		fmt.Println("Error sending HTTP request:", err)
		os.Exit(1)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		fmt.Println("Error: Unexpected response status code", resp.StatusCode)
		os.Exit(1)
	}

	contents, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		fmt.Println("Error reading response body:", err)
		os.Exit(1)
	}

	return string(contents)
}
