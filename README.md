# Cognitive Games

A collection of brain training games built with Rust and WebAssembly, focusing on numeracy and perception skills.

## Games

### Numeracy Game
A mathematical puzzle game that challenges your ability to compare numerical expressions.

**Features:**
- Select expressions in ascending order
- Dynamic difficulty scaling
- Time-based scoring with a 15-second round timer
- Level progression based on performance
- Supports basic arithmetic operations (+, -, ×, ÷)
- Decimal numbers in higher levels
- Progress auto-saving

### Perception Game (Maze)
A procedurally generated maze game testing spatial awareness and planning.

**Features:**
- Procedurally generated mazes using depth-first search
- Key-and-door mechanics
- Progressive difficulty with increasing maze size
- Move tracking
- 5-minute time limit per level
- Visual feedback for wall collisions
- Automatic progress saving
- Dark mode support

## Prerequisites

- Rust (nightly toolchain)
- Node.js
- npm

## Installation

```bash
# Clone the repository
git clone https://github.com/noneofyourbusiness1415252/cognitive-games.git
cd cognitive-games

# Install dependencies and build the project
npm install
npm run prepare
```

## Development

Start the development server:

```bash
npm start
```

The games will be accessible at:
- Numeracy Game: http://localhost:80/numeracy.html
- Maze Game: http://localhost:80/

## Building for Production

```bash
npm run build
```

The output will be in the `dist` directory.

## Testing

Run the test suite:

```bash
npm test
```

This will run both Rust unit tests and WebAssembly integration tests.

## Project Structure

- `src/games/numeracy/` - Numeracy game implementation
- `src/games/perception/` - Maze game implementation
- `static/` - HTML, CSS, and other static assets
- `js/` - JavaScript entry point
- `Cargo.toml` - Rust dependencies and configuration
- `package.json` - Node.js dependencies and scripts
- `webpack.config.js` - Webpack configuration

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Author

Umar Sharief