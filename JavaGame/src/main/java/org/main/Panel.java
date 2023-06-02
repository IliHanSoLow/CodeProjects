package org.main;

import org.input.KeyboardInput;
import org.input.MouseInput;
import org.vendor.opensimplex2.ImprovedNoise;
import org.vendor.opensimplex2.NoiseGenerator;
import org.vendor.opensimplex2.OpenSimplex2;

import javax.swing.*;
import java.awt.*;
import java.util.ArrayList;
import java.util.Random;

public class Panel extends JPanel {
    private final ArrayList<Line> lines = new ArrayList<Line>();
    double xSetOff = 0;
    Random random = new Random();
    Color color = Color.BLACK;
    Timer timer = new Timer(33, null);
    long seed = random.nextLong();
    private MouseInput mouseInput;
    private int xDelta = 0, yDelta = 0, xCoord = 0, yCoord = 0;

    public Panel() {
        mouseInput = new MouseInput(this);
        addKeyListener(new KeyboardInput(this));
        addMouseListener(mouseInput);
        addMouseMotionListener(mouseInput);
    }

    public void addLine(int x1, int y1, int x2, int y2) {
        this.lines.add(new Line(x1, y1, x2, y2));
    }

    public void changeXDelta(int value) {
        this.xDelta += value;
        repaint();
    }

    public void changeYDelta(int value) {
        this.yDelta += value;
        repaint();
    }

    public void increaseColor() {
        if (color == Color.RED) color = Color.GREEN;
        else if (color == Color.GREEN) color = Color.BLUE;
        else if (color == Color.BLUE) color = Color.RED;
        else color = Color.RED;
        repaint();
    }

    public void Loop() {
        while (true) {
            perlinNoise();
            timer.start();
        }
    }

    public void perlinNoise() {
//        long seed = random.nextLong();

        double xOff = xSetOff;
        int lastX = 0, lastY = 0;
        boolean ran = false;
        System.out.println("runs perlinnoise");
        for (int i = 0; i < getWidth(); i++) {
            for (int j = 0; j < getHeight(); j++) {
                double noiseValue = OpenSimplex2.noise2(seed, xOff, j);
                yCoord = j;
                xCoord = i;
                if (ran) {
                    addLine(lastX, lastY, xCoord, yCoord);
                    ran = false;
                }
                lastX = xCoord;
                lastY = yCoord;
                xOff += 0.01;
                ran = true;
                color = Color.getHSBColor(0, 0, (float) noiseValue);
                repaint();
            }
        }
        xSetOff += 0.01;

    }

    public void paintComponent(Graphics g) {
        System.out.println("paints");
        super.paintComponent(g);
//        g.fillRect(100 + xDelta, 10 + yDelta, 25, 25);
        g.setColor(color);
        g.drawRect(xCoord, yCoord, 1, 1);
    }

    public static class Line {
        public final int x1;
        public final int x2;
        public final int y1;
        public final int y2;

        public Line(int x1, int y1, int x2, int y2) {
            this.x1 = x1;
            this.x2 = x2;
            this.y1 = y1;
            this.y2 = y2;
        }

        public void paint(Graphics g) {
            g.drawLine(this.x1, this.y1, this.x2, this.y2);
        }
    }
}
