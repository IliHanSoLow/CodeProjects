/*
Copyright © 2023 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

var useerString string

// userCmd represents the user command
var userCmd = &cobra.Command{
	Use:   "user",
	Short: "Change or add User",
	Long:  `If you run this with the User flag you select your User. If the User doesnt exist a new one will be created`,
	Run: func(cmd *cobra.Command, args []string) {
		InitUser(useerString)
	},
}

func init() {
	rootCmd.AddCommand(userCmd)
	userCmd.Flags().StringVarP(&useerString, "user", "u", "", "The Username of the User being changed to")
	if err := userCmd.MarkFlagRequired("user"); err != nil {
		fmt.Println(err)
	}

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// userCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// userCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
