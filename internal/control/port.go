package control

import (
	"context"

	"reasonix/internal/agent"
	"reasonix/internal/event"
	"reasonix/internal/provider"
)

// This file defines the driving port: the typed, segregated interface surface
// that frontends (cli, desktop, bot, acp, serve) consume instead of coupling to
// the concrete *Controller and its ~99 methods. Each frontend depends only on
// the sub-ports it actually uses (interface segregation), so e.g. the bot never
// sees checkpoint or memory methods.
//
// The sub-ports are also the intended decomposition boundary for Controller
// itself: the port comes first and gives the later collaborator splits a spec to
// follow. *Controller implements every sub-port (asserted below). The full
// SessionAPI composition will accrete here as the remaining frontends migrate.

// Lifecycle covers a session's identity and lifecycle: minting, resuming,
// clearing, and locating the active session.
type Lifecycle interface {
	NewSession() error
	ClearSession() error
	Resume(s *agent.Session, path string)
	SetSessionPath(p string)
	SessionPath() string
	SessionDir() string
	Label() string
	WorkspaceRoot() string
	Close()
}

// TurnControl covers driving a model turn and observing its run state: the
// various submit/run entry points, cancellation, steering, and status reads.
type TurnControl interface {
	Submit(input string)
	SubmitDisplay(display, input string)
	SubmitHTTP(input string)
	SubmitUserTurn(input, display string)
	Send(input string)
	SendWithRaw(input, raw string)
	Run(ctx context.Context, input string) error
	RunTurn(ctx context.Context, input string) error
	RunShell(command string)
	Cancel()
	Steer(text string)
	SteerConsumed() bool
	Running() bool
	RuntimeStatus() RuntimeStatus
	Turn() int
	History() []provider.Message
	ToolResult(toolID string) *ToolResultData
}

// Approvals covers tool-approval and ask prompts plus the runtime approval
// posture (ask/auto/yolo). It mirrors the approvalManager surface.
type Approvals interface {
	Approve(id string, allow, session, persist bool)
	AnswerQuestion(id string, answers []event.AskAnswer)
	Ask(ctx context.Context, questions []event.AskQuestion) ([]event.AskAnswer, error)
	ReplayPendingPrompts()
	PendingPrompt() bool
	EnableInteractiveApproval()
	ToolApprovalMode() string
	SetToolApprovalMode(mode string)
	AutoApproveTools() bool
	SetAutoApproveTools(on bool)
	Bypass() bool
	SetBypass(on bool)
	SetMode(plan, autoApproveTools bool)
}

// Compile-time proof that the concrete controller satisfies each sub-port, so
// frontend migrations to the interfaces are mechanical and can never silently
// drift from the implementation.
var (
	_ Lifecycle   = (*Controller)(nil)
	_ TurnControl = (*Controller)(nil)
	_ Approvals   = (*Controller)(nil)
)
