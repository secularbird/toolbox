package main

import (
	"database/sql"
	"log"
	"net/http"
	"os"
	"strconv"
	"time"

	"github.com/gin-gonic/gin"
	_ "github.com/mattn/go-sqlite3"
)

type Reminder struct {
	ID          int64  `json:"id"`
	Title       string `json:"title" binding:"required"`
	Description string `json:"description"`
	Time        string `json:"time" binding:"required"`
	Completed   bool   `json:"completed"`
	Category    string `json:"category" binding:"required"`
	Frequency   string `json:"frequency" binding:"required"`
	CreatedAt   string `json:"created_at,omitempty"`
	UpdatedAt   string `json:"updated_at,omitempty"`
}

type Server struct {
	db     *sql.DB
	router *gin.Engine
}

func NewServer(dbPath string) (*Server, error) {
	// Open database
	db, err := sql.Open("sqlite3", dbPath)
	if err != nil {
		return nil, err
	}

	// Initialize database schema
	if err := initDatabase(db); err != nil {
		return nil, err
	}

	server := &Server{
		db:     db,
		router: gin.Default(),
	}

	// Setup routes
	server.setupRoutes()

	return server, nil
}

func initDatabase(db *sql.DB) error {
	query := `
	CREATE TABLE IF NOT EXISTS reminders (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		title TEXT NOT NULL,
		description TEXT NOT NULL,
		time TEXT NOT NULL,
		completed INTEGER NOT NULL DEFAULT 0,
		category TEXT NOT NULL,
		frequency TEXT NOT NULL,
		created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
		updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
	);

	CREATE INDEX IF NOT EXISTS idx_reminders_created_at ON reminders(created_at);
	CREATE INDEX IF NOT EXISTS idx_reminders_category ON reminders(category);
	`

	_, err := db.Exec(query)
	return err
}

func (s *Server) setupRoutes() {
	// Enable CORS
	s.router.Use(corsMiddleware())

	// Health check
	s.router.GET("/health", s.healthCheck)

	// API routes
	api := s.router.Group("/api")
	{
		api.GET("/reminders", s.getAllReminders)
		api.GET("/reminders/:id", s.getReminder)
		api.POST("/reminders", s.createReminder)
		api.PUT("/reminders/:id", s.updateReminder)
		api.DELETE("/reminders/:id", s.deleteReminder)
	}
}

func corsMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		c.Writer.Header().Set("Access-Control-Allow-Origin", "*")
		c.Writer.Header().Set("Access-Control-Allow-Credentials", "true")
		c.Writer.Header().Set("Access-Control-Allow-Headers", "Content-Type, Content-Length, Accept-Encoding, X-CSRF-Token, Authorization, accept, origin, Cache-Control, X-Requested-With")
		c.Writer.Header().Set("Access-Control-Allow-Methods", "POST, OPTIONS, GET, PUT, DELETE")

		if c.Request.Method == "OPTIONS" {
			c.AbortWithStatus(204)
			return
		}

		c.Next()
	}
}

func (s *Server) healthCheck(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"status":    "ok",
		"timestamp": time.Now().Unix(),
	})
}

func (s *Server) getAllReminders(c *gin.Context) {
	rows, err := s.db.Query(`
		SELECT id, title, description, time, completed, category, frequency, created_at, updated_at
		FROM reminders
		ORDER BY created_at DESC
	`)
	if err != nil {
		log.Printf("Error querying reminders: %v", err)
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to fetch reminders"})
		return
	}
	defer rows.Close()

	var reminders []Reminder
	for rows.Next() {
		var r Reminder
		var completed int
		err := rows.Scan(&r.ID, &r.Title, &r.Description, &r.Time, &completed, &r.Category, &r.Frequency, &r.CreatedAt, &r.UpdatedAt)
		if err != nil {
			log.Printf("Error scanning reminder: %v", err)
			continue
		}
		r.Completed = completed != 0
		reminders = append(reminders, r)
	}

	if reminders == nil {
		reminders = []Reminder{}
	}

	c.JSON(http.StatusOK, reminders)
}

func (s *Server) getReminder(c *gin.Context) {
	id := c.Param("id")

	var r Reminder
	var completed int
	err := s.db.QueryRow(`
		SELECT id, title, description, time, completed, category, frequency, created_at, updated_at
		FROM reminders
		WHERE id = ?
	`, id).Scan(&r.ID, &r.Title, &r.Description, &r.Time, &completed, &r.Category, &r.Frequency, &r.CreatedAt, &r.UpdatedAt)

	if err == sql.ErrNoRows {
		c.JSON(http.StatusNotFound, gin.H{"error": "Reminder not found"})
		return
	}
	if err != nil {
		log.Printf("Error querying reminder: %v", err)
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to fetch reminder"})
		return
	}

	r.Completed = completed != 0
	c.JSON(http.StatusOK, r)
}

func (s *Server) createReminder(c *gin.Context) {
	var reminder Reminder
	if err := c.ShouldBindJSON(&reminder); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	completed := 0
	if reminder.Completed {
		completed = 1
	}

	result, err := s.db.Exec(`
		INSERT INTO reminders (title, description, time, completed, category, frequency)
		VALUES (?, ?, ?, ?, ?, ?)
	`, reminder.Title, reminder.Description, reminder.Time, completed, reminder.Category, reminder.Frequency)

	if err != nil {
		log.Printf("Error creating reminder: %v", err)
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to create reminder"})
		return
	}

	id, _ := result.LastInsertId()
	reminder.ID = id

	log.Printf("Created reminder: id=%d, title=%s", id, reminder.Title)
	c.JSON(http.StatusCreated, reminder)
}

func (s *Server) updateReminder(c *gin.Context) {
	id := c.Param("id")
	idInt, err := strconv.ParseInt(id, 10, 64)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid ID"})
		return
	}

	var reminder Reminder
	if err := c.ShouldBindJSON(&reminder); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	completed := 0
	if reminder.Completed {
		completed = 1
	}

	result, err := s.db.Exec(`
		UPDATE reminders
		SET title = ?, description = ?, time = ?, completed = ?, category = ?, frequency = ?, updated_at = CURRENT_TIMESTAMP
		WHERE id = ?
	`, reminder.Title, reminder.Description, reminder.Time, completed, reminder.Category, reminder.Frequency, idInt)

	if err != nil {
		log.Printf("Error updating reminder: %v", err)
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to update reminder"})
		return
	}

	rowsAffected, _ := result.RowsAffected()
	if rowsAffected == 0 {
		c.JSON(http.StatusNotFound, gin.H{"error": "Reminder not found"})
		return
	}

	reminder.ID = idInt
	log.Printf("Updated reminder: id=%d, title=%s", idInt, reminder.Title)
	c.JSON(http.StatusOK, reminder)
}

func (s *Server) deleteReminder(c *gin.Context) {
	id := c.Param("id")

	result, err := s.db.Exec("DELETE FROM reminders WHERE id = ?", id)
	if err != nil {
		log.Printf("Error deleting reminder: %v", err)
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to delete reminder"})
		return
	}

	rowsAffected, _ := result.RowsAffected()
	if rowsAffected == 0 {
		c.JSON(http.StatusNotFound, gin.H{"error": "Reminder not found"})
		return
	}

	log.Printf("Deleted reminder: id=%s", id)
	c.JSON(http.StatusOK, gin.H{"message": "Reminder deleted successfully"})
}

func (s *Server) Run(port string) error {
	log.Printf("Starting sync server on port %s", port)
	return s.router.Run(":" + port)
}

func (s *Server) Close() error {
	return s.db.Close()
}

func main() {
	// Get configuration from environment variables
	port := os.Getenv("PORT")
	if port == "" {
		port = "3000"
	}

	dbPath := os.Getenv("DB_PATH")
	if dbPath == "" {
		dbPath = "./sync_server.db"
	}

	// Create server
	server, err := NewServer(dbPath)
	if err != nil {
		log.Fatalf("Failed to create server: %v", err)
	}
	defer server.Close()

	// Run server
	if err := server.Run(port); err != nil {
		log.Fatalf("Failed to run server: %v", err)
	}
}
