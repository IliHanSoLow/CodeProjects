/*
Copyright © 2023 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"github.com/spf13/cobra"
)

// addCmd represents the add command
var addCmd = &cobra.Command{
	Use:   "add",
	Short: "add a Task",
	Long:  ``,
	Run: func(cmd *cobra.Command, args []string) {
		newTask(newTitle, newPrio, newDescription)
	},
}

func init() {
	rootCmd.AddCommand(addCmd)
	addCmd.Flags().IntVarP(&newPrio, "priority", "p", 0, "The new Priority.")
	addCmd.Flags().StringVarP(&newTitle, "title", "t", "", "The new Title")
	addCmd.Flags().StringVarP(&newDescription, "description", "d", "", "The new description")

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// addCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// addCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
