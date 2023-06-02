/*
Copyright © 2023 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"github.com/spf13/cobra"
)

var (
	tasknum        int
	newTitle       string
	newDescription string
	newPrio        int
)

// taskCmd represents the task command
var taskCmd = &cobra.Command{
	Use:   "edit",
	Short: "edit tasks",
	Long:  ``,
	Run: func(cmd *cobra.Command, args []string) {
		editTask(tasknum, newTitle, newPrio, newDescription)
	},
}

func init() {
	rootCmd.AddCommand(taskCmd)

	userCmd.Flags().IntVarP(&tasknum, "place", "p", 0, "The Place of the Task wanted to be changed.")
	userCmd.Flags().IntVarP(&newPrio, "priority", "y", 0, "The new Priority.")
	userCmd.Flags().StringVarP(&newTitle, "title", "t", "", "The new Title")
	userCmd.Flags().StringVarP(&newDescription, "description", "d", "", "The new description")
	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// taskCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// taskCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
