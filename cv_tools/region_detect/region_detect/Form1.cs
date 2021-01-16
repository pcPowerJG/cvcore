using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace region_detect
{
    public partial class Form1 : Form
    {
        public struct point
        {
            public int x;
            public int y;
        }

        string image_path = "";

        point p1;
        point p2;
        point p3;
        point p4;

        bool p1b;
        bool p2b;
        bool p3b;
        bool p4b;

        Graphics g;
        public Form1()
        {
            InitializeComponent();
            pictureBox1.SizeMode = PictureBoxSizeMode.StretchImage;
            g = pictureBox1.CreateGraphics();
        }

        private void button1_Click(object sender, EventArgs e)
        {
            //1
            p1b = true;
            p2b = p3b = p4b = false;
            label2.Text = "1";
        }

        private void button2_Click(object sender, EventArgs e)
        {
            //2
            p2b = true;
            p1b = p3b = p4b = false;
            label2.Text = "2";
        }

        private void button4_Click(object sender, EventArgs e)
        {
            //3
            p3b = true;
            p2b = p1b = p4b = false;
            label2.Text = "3";
        }

        private void button3_Click(object sender, EventArgs e)
        {
            //4
            p4b = true;
            p1b = p3b = p2b = false;
            label2.Text = "4";
        }

        private void button5_Click(object sender, EventArgs e)
        {
            //save
            System.IO.File.Create(image_path + ".txt").Close();
            try
            {
                using (StreamWriter sw = new StreamWriter(image_path + ".txt", false, System.Text.Encoding.UTF8))
                {
                    sw.WriteLine(p1.x.ToString() + "," + p1.y.ToString() + "," + p4.x.ToString() + "," + p4.y.ToString());
                }
            }
            catch (Exception err)
            {
                MessageBox.Show(err.Message);
            }
        }

        private void button6_Click(object sender, EventArgs e)
        {
            //load
            OpenFileDialog fd = new OpenFileDialog();
            fd.ShowDialog();
            image_path = fd.FileName;
            pictureBox1.Image = Image.FromFile(image_path);
        }

        private void button7_Click(object sender, EventArgs e)
        {
            //reset load
            pictureBox1.Image = Image.FromFile(image_path);
            p1b = p3b = p2b = p4b = false;
        }

        private void pictureBox1_MouseDown(object sender, MouseEventArgs e)
        {
            int x = e.X;
            int y = e.Y;
            if (p1b)
            {
                g.DrawRectangle(new Pen(Brushes.Red), new Rectangle(x, y, 4, 4));
                p1.x = x;
                p1.y = y;
            }
            else if (p2b)
            {
                g.DrawRectangle(new Pen(Brushes.Pink), new Rectangle(x, y, 4, 4));
                p2.x = x;
                p2.y = y;
            }
            else if (p3b)
            {
                g.DrawRectangle(new Pen(Brushes.Blue), new Rectangle(x, y, 4, 4));
                p3.x = x;
                p3.y = y;
            }
            else if (p4b)
            {
                g.DrawRectangle(new Pen(Brushes.Green), new Rectangle(x, y, 4, 4));
                p4.x = x;
                p4.y = y;
            }
        }

        private void button8_Click(object sender, EventArgs e)
        {
            g.DrawRectangle(new Pen(Brushes.Green), new Rectangle(p1.x, p1.y, (p4.x - p1.x), (p4.y - p1.y)));
        }
    }
}
