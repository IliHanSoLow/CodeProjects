/*
Copyright © 2023 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"github.com/spf13/cobra"
)

var taskpos int

// completedCmd represents the completed command
var completedCmd = &cobra.Command{
	Use:   "completed",
	Short: "Toggle Completion of a task",
	Long:  ``,
	Run: func(cmd *cobra.Command, args []string) {
		completeTask(taskpos)
	},
}

func init() {
	completedCmd.Flags().IntVarP(&taskpos, "palce", "p", 0, "The Place of the task that should be changed")
	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// completedCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// completedCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
